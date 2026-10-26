// Deferred original/native product producer.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-fresh-vector-results.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-fresh-vector-result-program'));
