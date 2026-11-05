import {produceJavaScriptProgramOracle} from '../javascript-program/oracle-source.mjs';
import {createBigIntegerConversionProgramLinks} from '../../js/src/big-integer-conversion-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/big-integer-source.lino',import.meta.url),createBigIntegerConversionProgramLinks);
