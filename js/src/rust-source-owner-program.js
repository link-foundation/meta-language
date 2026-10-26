// Full checked source owners use the existing typed Links schema and codecs.
import {parseRustWithSourceTypeOwners} from './translation/rust.js';
import {checkProgram} from './translation/check.js';
import {encodeTypedProgramLinks} from './typed-program-links.js';
export function createRustSourceOwnerProgramLinks(source){return encodeTypedProgramLinks(checkProgram(parseRustWithSourceTypeOwners(source)));}
