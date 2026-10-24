import {produceJavaScriptProgramOracle} from '../javascript-program/oracle-source.mjs';
import {createStringNumberProgramLinks} from '../../js/src/string-number-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/string-number.lino',import.meta.url),createStringNumberProgramLinks);
