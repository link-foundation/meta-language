// Data-field positions reuse the complete source type-owner interpretation.
/** @param {string} model @param {string} language @param {string} declared @param {string} actual @param {number} constructors @param {number} expected @returns {boolean} */
export function acceptRustDataTypeFieldSignature(model,language,declared,actual,constructors,expected){
 return model==='Rust data type field signatures 1'&&language==='Rust'&&declared===actual&&constructors===expected&&acceptRustDataTypeFieldSignatureBudget(constructors,0)&&acceptRustDataTypeFieldSignatureBudget(expected,0);
}
/** @param {string} name @param {string} actual @param {number} fields @param {number} expected @returns {boolean} */
export function acceptRustConstructorFieldSignature(name,actual,fields,expected){return name===actual&&fields===expected&&acceptRustDataTypeFieldSignatureBudget(fields,0)&&acceptRustDataTypeFieldSignatureBudget(expected,0);}
/** @param {number} visits @param {number} depth @returns {boolean} */
export function acceptRustDataTypeFieldSignatureBudget(visits,depth){return Number.isInteger(visits)&&visits>=0&&visits<=100000&&Number.isInteger(depth)&&depth>=0&&depth<=256;}
/** @param {string} kind @param {boolean} dense @param {boolean} unrepresented @returns {boolean} */
export function acceptRustDataTypeFieldSignatureContainer(kind,dense,unrepresented){return !unrepresented&&(kind==='record'||kind==='array'&&dense);}
/** @param {string} kind @param {boolean} dense @param {boolean} unrepresented @returns {boolean} */
export function acceptRustDataTypeFieldSignatureProgramContainer(kind,dense,unrepresented){return kind==='map'?!unrepresented:acceptRustDataTypeFieldSignatureContainer(kind,dense,unrepresented);}
