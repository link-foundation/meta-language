import {readSourceVersionColumns} from './source-version-column-rules.js';
// Mechanical decimal version facts only; supported ranges are shared rules.
import {unsupported} from './diagnostics.js';
import {readRustSourceVersionDefault,readRustSourceVersionError,readRustSourceVersionTextLimit} from './rust-source-version-rules.js';
export function readRustSourceVersion(version=''){
 if(typeof version!=='string')throw new TypeError('Rust sourceVersion must be a string');
 if(version.length>readRustSourceVersionTextLimit())throw unsupported('Rust source version',readRustSourceVersionError(-1,-1,-1,false));
 const spelling=version===''?readRustSourceVersionDefault():version,parts=readSourceVersionColumns(spelling),canonical=parts.length===3,[major,minor,patch]=parts;
 const error=readRustSourceVersionError(major??-1,minor??-1,patch??-1,canonical);if(error)throw unsupported('Rust source version',error);
 return {spelling,major,minor,patch};
}
