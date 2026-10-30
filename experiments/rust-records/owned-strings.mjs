// Deferred complete original/generated products. Never run during drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-owned-strings.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-owned-binding-program'));
