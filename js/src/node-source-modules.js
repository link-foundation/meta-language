// Explicit Node embedding entry point. Represented bodies execute only in the
// portable machine; this module coordinates its qualified external operations.
export {readSuppliedSourceLibraryClosure} from './translation/source-library-closure.js';
export {captureSourceLibraryErrorClosure,checkSourceLibraryErrorClosure} from './translation/javascript-error-closure-source.js';
export {prepareJavaScriptErrorSourceClosure,beginJavaScriptErrorSourceClosure,advanceJavaScriptErrorSourceClosure,resumeJavaScriptErrorSourceClosure} from './translation/javascript-error-closure.js';
export {captureJavaScriptRuntimeDataModule,checkJavaScriptRuntimeDataModule} from './translation/runtime-data-module-source.js';
export {readActiveRuntimeDataVersions,readShippedRuntimeDataAdapterContract,createRuntimeDataModuleScope,prepareJavaScriptRuntimeDataModule} from './translation/runtime-data-module.js';
export {captureJavaScriptProjectDataSource,checkJavaScriptProjectDataSource} from './translation/javascript-project-data-source.js';
export {readShippedJavaScriptProjectDataSource,prepareJavaScriptProjectDataModule} from './translation/javascript-project-data-module.js';
export {captureJavaScriptOrdinaryClassModule,checkJavaScriptOrdinaryClassModule} from './translation/javascript-ordinary-class-source.js';
export {prepareJavaScriptOrdinaryClassModule} from './translation/javascript-ordinary-class-module.js';
export {captureJavaScriptOrdinaryClassSourceClosure,checkJavaScriptOrdinaryClassSourceClosure} from './translation/javascript-ordinary-class-closure-source.js';
export {prepareJavaScriptOrdinaryClassSourceClosure} from './translation/javascript-ordinary-class-closure.js';
export {runJavaScriptSourceTransition,resumeJavaScriptSourceTransition} from './translation/javascript-source-driver.js';
