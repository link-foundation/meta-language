// Deferred original/generated products. Never execute during drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-byte-string-expressions.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-byte-string-expression-program'));
