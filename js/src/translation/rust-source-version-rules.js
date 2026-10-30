// Stable source std snapshots are independent of the emitted target toolchain.
/** @returns {string} */
export function readRustSourceVersionDefault(){return '1.99.0';}
/** @param {number} major @param {number} minor @param {number} patch
 * @param {boolean} canonical @returns {string} */
export function readRustSourceVersionError(major,minor,patch,canonical){return canonical&&major===1&&Number.isInteger(minor)&&minor>=90&&minor<100&&Number.isInteger(patch)&&patch>=0&&patch<=4294967295?'':'select a canonical stable Rust source version from 1.90.0 through 1.99.x';}
/** @param {number} minor @returns {string} */
export function readRustIntegerConversionErrorDebugModel(minor){return Number.isInteger(minor)&&minor>=90&&minor<97?'unit payload':Number.isInteger(minor)&&minor>=97&&minor<100?'overflow kind':'';}

/** @returns {number} */
export function readRustSourceVersionTextLimit(){return 15;}
