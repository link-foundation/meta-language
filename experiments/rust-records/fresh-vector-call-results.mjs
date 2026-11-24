// Future complete original/native products. Do not run while drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-fresh-vector-call-results.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-fresh-vector-call-result-program'));
