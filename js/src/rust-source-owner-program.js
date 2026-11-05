import {readRustSourceVersion} from './translation/rust-source-versions.js';
// Full checked source owners use the existing typed Links schema and codecs.
import {parseRustWithSourceTypeOwners} from './translation/rust.js';
import {checkProgram} from './translation/check.js';
import {encodeTypedProgramLinks} from './typed-program-links.js';
export function createRustSourceOwnerProgramLinks(source,options={}){const sourceVersion=options.sourceVersion,selected=sourceVersion!==undefined;return encodeTypedProgramLinks(checkProgram(parseRustWithSourceTypeOwners(source,{sourceVersion})),selected?{provenance:{sourceProfile:{language:'Rust',rustSourceVersion:readRustSourceVersion(sourceVersion).spelling},userProvenance:null}}:{});}
