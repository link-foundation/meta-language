import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-string-byte-lengths.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-string-byte-length-program'));
