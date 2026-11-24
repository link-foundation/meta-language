// Deferred complete original/generated products. Never run during drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-iterator-arithmetic.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-iterator-arithmetic-program'));
