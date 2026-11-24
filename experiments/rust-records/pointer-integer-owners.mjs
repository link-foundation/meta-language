// Future complete original/native products; no execution while drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-pointer-integer-owners.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-pointer-integer-owner-program'));
