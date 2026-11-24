// Standard operation identifiers shared by call compilation and execution.
// The identifiers describe instructions. Realm directories supply actual callable authority.
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardArrayCreationOperation(operation){return operation==='Array.of'||operation==='Array.from';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardObjectFromEntriesOperation(operation){return operation==='Object.fromEntries';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardCollectionForEachOperation(operation){return operation==='Map.forEach'||operation==='Set.forEach';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardObjectObservationOperation(operation){return ['Object.builtinTag','Symbol.valueOf','Number.valueOf','Boolean.valueOf','Boolean.toString','BigInt.valueOf','String.toString','String.valueOf','Object.regularExpressionSlot','Object.prototype.hasOwnProperty','Object.prototype.propertyIsEnumerable','Object.hasOwn','Object.is','Object.isExtensible','Object.isSealed','Object.isFrozen','Object.preventExtensions','Object.seal','Object.freeze','Object.getPrototypeOf','Object.setPrototypeOf'].includes(operation);}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardArrayIntrinsicOperation(operation){return operation==='Array.isArray';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardArrayCallbackOperation(operation){return ['Array.forEach','Array.some','Array.every','Array.find','Array.findIndex','Array.findLast','Array.findLastIndex','Array.reduce','Array.reduceRight'].includes(operation);}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardArrayTransformOperation(operation){return operation==='Array.map'||operation==='Array.filter';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardArraySpeciesGetterOperation(operation){return operation==='Array.species getter';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardObjectEnumerationOperation(operation){return ['Object.keys','Object.values','Object.entries'].includes(operation);}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardPropertyReflectionOperation(operation){return ['Object.getOwnPropertyDescriptor','Object.getOwnPropertyDescriptors','Object.getOwnPropertyNames','Object.getOwnPropertySymbols','Reflect.ownKeys','Reflect.getOwnPropertyDescriptor'].includes(operation);}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardSingleDescriptorOperation(operation){return operation==='Object.getOwnPropertyDescriptor'||operation==='Reflect.getOwnPropertyDescriptor';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardPropertyDefinitionsOperation(operation){return operation==='Object.defineProperties'||operation==='Object.create';}
/** @param {string} operation @returns {boolean} */
export function testJavaScriptStandardPrimitiveMethodOperation(operation){return operation==='String.startsWith';}
