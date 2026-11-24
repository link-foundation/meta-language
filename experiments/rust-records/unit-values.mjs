import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createRustRecordProgramLinks} from '../../js/src/rust-record-program.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-unit-values.lino',import.meta.url),createRustRecordProgramLinks);
