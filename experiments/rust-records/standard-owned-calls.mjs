// Deferred original/generated product matrix; do not execute during drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-standard-owned-calls.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-standard-owned-call-program'));
