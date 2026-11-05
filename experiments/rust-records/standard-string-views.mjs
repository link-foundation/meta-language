// Deferred complete original/generated products; never execute while drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-standard-string-views.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-standard-string-view-program'));
