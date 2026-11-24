// Deferred full original/native products; no execution while drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-source-owner-retention.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-source-owner-retention-program'));
