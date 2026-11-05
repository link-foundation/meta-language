// A standard string view can be consumed only by an actual nonescaping call.
/** @param {string} path @param {boolean} ordinary @param {boolean} shadowed @returns {boolean} */
export function acceptRustStandardStringViewPath(path,ordinary,shadowed){return ordinary&&!shadowed&&path==='std::string::String::as_str';}
/** @param {number} layers @returns {boolean} */
export function acceptRustStandardStringViewLayers(layers){return Number.isInteger(layers)&&layers>=1&&layers<=256;}
/** @param {string} parameter @param {number} layers @param {boolean} knownSlice @param {boolean} staticLifetime @param {string} result @param {number} resultLayers @param {boolean} knownResult @returns {boolean} */
export function acceptRustStandardStringViewConsumer(parameter,layers,knownSlice,staticLifetime,result,resultLayers,knownResult){return parameter==='string'&&acceptRustStandardStringViewLayers(layers)&&knownSlice&&!staticLifetime&&resultLayers===0&&knownResult&&(result==='string'||result==='fixed'||result==='float'||result==='bool'||result==='unit');}
/** @param {string} kind @param {string} model @param {number} layers @param {boolean} proven @param {boolean} references @param {boolean} known @param {number} argumentsCount @returns {boolean} */
export function acceptRustStandardStringViewReceiver(kind,model,layers,proven,references,known,argumentsCount){return kind==='string'&&model==='owned string'&&layers===1&&proven&&references&&known&&argumentsCount===1;}
/** @param {number} sites @param {boolean} root @param {string} kind @param {number} layers @param {number} expected @param {boolean} references @returns {boolean} */
export function acceptRustStandardStringViewArgument(sites,root,kind,layers,expected,references){return sites===1&&root&&kind==='string'&&layers===expected&&acceptRustStandardStringViewLayers(layers)&&references;}
