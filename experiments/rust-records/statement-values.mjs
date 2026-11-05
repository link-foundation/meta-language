import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createRustUnitValueProgramLinks} from '../../js/src/rust-unit-value-program.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-statement-values.lino',import.meta.url),createRustUnitValueProgramLinks);
