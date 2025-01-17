//! Subcall dispatch.
use std::cell::RefCell;

use crate::{
    context::{BatchContext, Context, State, TransactionWithMeta, TxContext},
    dispatcher,
    module::CallResult,
    modules::core::{Error, API as _},
    runtime::Runtime,
    storage::{current::TransactionResult, CurrentStore},
    types::{token, transaction, transaction::CallerAddress},
};

thread_local! {
    /// The subcall stack for tracking depth and other metadata.
    static SUBCALL_STACK: RefCell<SubcallStack> = RefCell::new(SubcallStack::new());
}

/// Subcall validator.
pub trait Validator {
    /// Validate a subcall before it is performed.
    fn validate(&self, info: &SubcallInfo) -> Result<(), Error>;
}

/// A validator which allows everything.
pub struct AllowAllValidator;

impl Validator for AllowAllValidator {
    fn validate(&self, _info: &SubcallInfo) -> Result<(), Error> {
        Ok(())
    }
}

/// Information about a subcall to be dispatched.
#[derive(Clone, Debug)]
pub struct SubcallInfo {
    /// Address of the caller.
    pub caller: CallerAddress,
    /// Method to call.
    pub method: String,
    /// Subcall body.
    pub body: cbor::Value,
    /// Maximum subcall depth.
    pub max_depth: u16,
    /// Maximum gas amount that can be consumed.
    pub max_gas: u64,
}

/// Result of dispatching a subcall.
#[derive(Debug)]
pub struct SubcallResult {
    /// State after applying the subcall context.
    pub state: State,
    /// Result of the subcall.
    pub call_result: CallResult,
    /// Gas used by the subcall.
    pub gas_used: u64,
}

struct SubcallStackEntry {
    validator: Box<dyn Validator>,
}

struct SubcallStack {
    stack: Vec<SubcallStackEntry>,
}

impl SubcallStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn depth(&self) -> u16 {
        self.stack.len() as u16
    }

    fn push(&mut self, entry: SubcallStackEntry) {
        self.stack.push(entry);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn run_validators(&self, info: &SubcallInfo) -> Result<(), Error> {
        for entry in &self.stack {
            entry.validator.validate(info)?;
        }
        Ok(())
    }
}

struct SubcallStackGuard;

impl Drop for SubcallStackGuard {
    fn drop(&mut self) {
        SUBCALL_STACK.with(|ss| {
            ss.borrow_mut().pop();
        });
    }
}

/// The current subcall depth.
pub fn get_current_subcall_depth<C: Context>(_ctx: &mut C) -> u16 {
    SUBCALL_STACK.with(|ss| ss.borrow().depth())
}

/// Perform a subcall.
pub fn call<C: TxContext, V: Validator + 'static>(
    ctx: &mut C,
    info: SubcallInfo,
    validator: V,
) -> Result<SubcallResult, Error> {
    // Run validator first.
    validator.validate(&info)?;

    // Update the subcall stack after doing validation.
    SUBCALL_STACK.with(|ss| {
        let mut stack = ss.borrow_mut();

        // Ensure the call depth is not too large.
        if stack.depth() >= info.max_depth {
            return Err(Error::CallDepthExceeded(stack.depth() + 1, info.max_depth));
        }

        // Run existing validators.
        stack.run_validators(&info)?;

        // Push subcall to stack.
        stack.push(SubcallStackEntry {
            validator: Box::new(validator) as Box<dyn Validator>,
        });

        Ok(())
    })?;
    let _guard = SubcallStackGuard; // Ensure subcall is popped from stack.

    // Calculate how many consensus messages the child call can emit.
    let remaining_messages = ctx.remaining_messages();

    // Execute a transaction in a child context.
    let (call_result, gas, state) = ctx.with_child(ctx.mode(), |mut ctx| {
        // Generate an internal transaction.
        let tx = transaction::Transaction {
            version: transaction::LATEST_TRANSACTION_VERSION,
            call: transaction::Call {
                format: transaction::CallFormat::Plain,
                method: info.method,
                body: info.body,
                ..Default::default()
            },
            auth_info: transaction::AuthInfo {
                signer_info: vec![transaction::SignerInfo {
                    // The call is being performed on the caller's behalf.
                    address_spec: transaction::AddressSpec::Internal(info.caller),
                    nonce: 0,
                }],
                fee: transaction::Fee {
                    amount: token::BaseUnits::new(0, token::Denomination::NATIVE),
                    // Limit gas usage inside the child context to the allocated maximum.
                    gas: info.max_gas,
                    consensus_messages: remaining_messages,
                },
                ..Default::default()
            },
        };

        let result = CurrentStore::with_transaction(|| {
            ctx.with_tx(TransactionWithMeta::internal(tx), |ctx, call| {
                // Mark this sub-context as internal as it belongs to an existing transaction.
                let mut ctx = ctx.internal();

                // Dispatch the call.
                let (result, _) = dispatcher::Dispatcher::<C::Runtime>::dispatch_tx_call(
                    &mut ctx,
                    call,
                    &Default::default(),
                );
                // Retrieve remaining gas.
                let gas = <C::Runtime as Runtime>::Core::remaining_tx_gas(&mut ctx);

                // Commit store and return emitted tags and messages on successful dispatch,
                // otherwise revert state and ignore any emitted events/messages.
                if result.is_success() {
                    let state = ctx.commit();
                    TransactionResult::Commit((result, gas, state))
                } else {
                    // Ignore tags/messages on failure.
                    TransactionResult::Rollback((result, gas, Default::default()))
                }
            })
        });

        // Commit. Note that if child context didn't commit, this is basically a no-op.
        ctx.commit();

        result
    });

    // Compute the amount of gas used.
    let gas_used = info.max_gas.saturating_sub(gas);

    Ok(SubcallResult {
        state,
        call_result,
        gas_used,
    })
}
