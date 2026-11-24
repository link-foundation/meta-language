/** @param {string} profile @returns {string} */
export function readJavaScriptModuleInitializationProfileError(profile){return profile==='actual runtime Links policy initializers 1'?'':'runtime policy modules require their actual source-bound IO initialization profile';}
/** External qualification, not proof that IO/decoder source bodies have been
 * translated. Uncovered engines/dependencies are explicit unsupported cells.
 * @param {number} nodeMajor @param {number} nodeMinor @param {string} linksVersion @returns {string} */
export function readJavaScriptRuntimeDataVersionError(nodeMajor,nodeMinor,linksVersion){return Number.isSafeInteger(nodeMajor)&&Number.isSafeInteger(nodeMinor)&&nodeMinor>=0&&(nodeMajor===24||nodeMajor===22&&nodeMinor>=15)&&linksVersion==='0.22.0'?'':'runtime data IO has uncovered Node or links-notation versions';}
/** @param {string} phase @returns {boolean} */
export function testJavaScriptModuleInitializationPhase(phase){return ['uninitialized','initializing','loading','temporal dead zone','evaluated','errored'].includes(phase);}
/** @param {string} phase @param {boolean} initialized @returns {number} */
export function readJavaScriptModuleBindingRead(phase,initialized){return phase==='errored'?-2:initialized?1:0;}
