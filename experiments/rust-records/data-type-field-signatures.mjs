// Future original/generated product matrix; never execute during drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-data-type-field-signatures.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-data-type-field-signature-program'));
