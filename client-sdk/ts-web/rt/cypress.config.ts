import {defineConfig} from 'cypress';
import * as outputConsoleLogs from './../core/cypress/outputConsoleLogs';

export default defineConfig({
    video: false,
    e2e: {
        supportFile: false,
        setupNodeEvents: outputConsoleLogs.setupNodeEvents,
    },
});
