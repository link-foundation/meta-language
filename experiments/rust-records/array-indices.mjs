// Deferred complete original/native source producer; do not run while drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-array-indices.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-array-index-program'));
