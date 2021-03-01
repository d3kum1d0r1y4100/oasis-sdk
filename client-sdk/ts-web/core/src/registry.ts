import * as misc from './misc';
import * as signature from './signature';
import * as types from './types';

/**
 * RegisterEntitySignatureContext is the context used for entity
 * registration.
 */
export const REGISTER_ENTITY_SIGNATURE_CONTEXT = 'oasis-core/registry: register entity';
/**
 * RegisterGenesisEntitySignatureContext is the context used for
 * entity registration in the genesis document.
 *
 * Note: This is identical to non-gensis registrations to support
 * migrating existing registrations into a new genesis document.
 */
export const REGISTER_GENESIS_ENTITY_SIGNATURE_CONTEXT = REGISTER_ENTITY_SIGNATURE_CONTEXT;
/**
 * RegisterNodeSignatureContext is the context used for node
 * registration.
 */
export const REGISTER_NODE_SIGNATURE_CONTEXT = 'oasis-core/registry: register node';
/**
 * RegisterGenesisNodeSignatureContext is the context used for
 * node registration in the genesis document.
 *
 * Note: This is identical to non-gensis registrations to support
 * migrating existing registrations into a new genesis document.
 */
export const REGISTER_GENESIS_NODE_SIGNATURE_CONTEXT = REGISTER_NODE_SIGNATURE_CONTEXT;

/**
 * MethodRegisterEntity is the method name for entity registrations.
 */
export const METHOD_REGISTER_ENTITY = 'registry.RegisterEntity';
/**
 * MethodDeregisterEntity is the method name for entity deregistrations.
 */
export const METHOD_DEREGISTER_ENTITY = 'registry.DeregisterEntity';
/**
 * MethodRegisterNode is the method name for node registrations.
 */
export const METHOD_REGISTER_NODE = 'registry.RegisterNode';
/**
 * MethodUnfreezeNode is the method name for unfreezing nodes.
 */
export const METHOD_UNFREEZE_NODE = 'registry.UnfreezeNode';
/**
 * MethodRegisterRuntime is the method name for registering runtimes.
 */
export const METHOD_REGISTER_RUNTIME = 'registry.RegisterRuntime';

/**
 * GasOpRegisterEntity is the gas operation identifier for entity registration.
 */
export const GAS_OP_REGISTER_ENTITY = 'register_entity';
/**
 * GasOpDeregisterEntity is the gas operation identifier for entity deregistration.
 */
export const GAS_OP_DEREGISTER_ENTITY = 'deregister_entity';
/**
 * GasOpRegisterNode is the gas operation identifier for entity registration.
 */
export const GAS_OP_REGISTER_NODE = 'register_node';
/**
 * GasOpUnfreezeNode is the gas operation identifier for unfreezing nodes.
 */
export const GAS_OP_UNFREEZE_NODE = 'unfreeze_node';
/**
 * GasOpRegisterRuntime is the gas operation identifier for runtime registration.
 */
export const GAS_OP_REGISTER_RUNTIME = 'register_runtime';
/**
 * GasOpRuntimeEpochMaintenance is the gas operation identifier for per-epoch
 * runtime maintenance costs.
 */
export const GAS_OP_RUNTIME_EPOCH_MAINTENANCE = 'runtime_epoch_maintenance';
/**
 * GasOpUpdateKeyManager is the gas operation identifier for key manager
 * policy updates costs.
 */
export const GAS_OP_UPDATEKEY_MANAGER = 'update_keymanager';

/**
 * KindInvalid is an invalid runtime and should never be explicitly set.
 */
export const KIND_INVALID = 0;
/**
 * KindCompute is a generic compute runtime.
 */
export const KIND_COMPUTE = 1;
/**
 * KindKeyManager is a key manager runtime.
 */
export const KIND_KEY_MANAGER = 2;

export const GOVERNANCE_INVALID = 0;
export const GOVERNANCE_ENTITY = 1;
export const GOVERNANCE_RUNTIME = 2;
export const GOVERNANCE_CONSENSUS = 3;
export const GOVERNANCE_MAX = GOVERNANCE_CONSENSUS;

/**
 * LatestRuntimeDescriptorVersion is the latest entity descriptor version that should be used
 * for all new descriptors. Using earlier versions may be rejected.
 */
export const LATEST_RUNTIME_DESCRIPTOR_VERSION = 2;

/**
 * ModuleName is a unique module name for the registry module.
 */
export const MODULE_NAME = 'registry';
/**
 * ErrInvalidArgument is the error returned on malformed argument(s).
 */
export const CODE_INVALID_ARGUMENT = 1;
/**
 * ErrInvalidSignature is the error returned on an invalid signature.
 */
export const CODE_INVALID_SIGNATURE = 2;
/**
 * ErrBadEntityForNode is the error returned when a node registration
 * with an unknown entity is attempted.
 */
export const CODE_BAD_ENTITY_FOR_NODE = 3;
/**
 * ErrBadEntityForRuntime is the error returned when a runtime
 * attempts to register with an unknown entity.
 */
export const CODE_BAD_ENTITY_FOR_RUNTIME = 4;
/**
 * ErrNoEnclaveForRuntime is the error returned when a TEE runtime
 * registers with no enclave IDs.
 */
export const CODE_NO_ENCLAVE_FOR_RUNTIME = 5;
/**
 * ErrBadEnclaveIdentity is the error returned when a node tries to
 * register runtimes with wrong Enclave IDs.
 */
export const CODE_BAD_ENCLAVE_IDENTITY = 6;
/**
 * ErrBadCapabilitiesTEEHardware is the error returned when a node tries to
 * register a runtime with bad Capabilities.TEE.Hardware.
 */
export const CODE_BAD_CAPABILITIES_TEE_HARDWARE = 7;
/**
 * ErrTEEHardwareMismatch is the error returned when a node tries to
 * register a runtime and Capabilities.TEE.Hardware mismatches the one in
 * the registry.
 */
export const CODE_TEE_HARDWARE_MISMATCH = 8;
/**
 * ErrNoSuchEntity is the error returned when an entity does not exist.
 */
export const CODE_NO_SUCH_ENTITY = 9;
/**
 * ErrNoSuchNode is the error returned when an node does not exist.
 */
export const CODE_NO_SUCH_NODE = 10;
/**
 * ErrNoSuchRuntime is the error returned when an runtime does not exist.
 */
export const CODE_NO_SUCH_RUNTIME = 11;
/**
 * ErrIncorrectTxSigner is the error returned when the signer of the transaction
 * is not the correct one.
 */
export const CODE_INCORRECT_TX_SIGNER = 12;
/**
 * ErrNodeExpired is the error returned when a node is expired.
 */
export const CODE_NODE_EXPIRED = 13;
/**
 * ErrNodeCannotBeUnfrozen is the error returned when a node cannot yet be
 * unfrozen due to the freeze period not being over yet.
 */
export const CODE_NODE_CANNOT_BE_UNFROZEN = 14;
/**
 * ErrEntityHasNodes is the error returned when an entity cannot be deregistered
 * as it still has nodes.
 */
export const CODE_ENTITY_HAS_NODES = 15;
/**
 * ErrForbidden is the error returned when an operation is forbidden by
 * policy.
 */
export const CODE_FORBIDDEN = 16;
/**
 * ErrNodeUpdateNotAllowed is the error returned when trying to update an existing node with
 * disallowed changes.
 */
export const CODE_NODE_UPDATE_NOT_ALLOWED = 17;
/**
 * ErrRuntimeUpdateNotAllowed is the error returned when trying to update an existing runtime.
 */
export const CODE_RUNTIME_UPDATE_NOT_ALLOWED = 18;
/**
 * ErrEntityHasRuntimes is the error returned when an entity cannot be deregistered as it still
 * has runtimes.
 */
export const CODE_ENTITY_HAS_RUNTIMES = 19;
