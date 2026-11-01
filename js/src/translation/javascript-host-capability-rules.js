/** A receiver mode is a code restriction, not proof of host authority.
 * The external provider must qualify the actual source/operation owner.
 * @param {string} mode @returns {boolean} */
export function testJavaScriptHostCapabilityMode(mode){return mode==='qualified host capability';}
