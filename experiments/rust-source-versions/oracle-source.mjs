// Deferred common/default original product controls; no source version executes here.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-source-versions.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-source-version-program'));
