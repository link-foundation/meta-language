import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptStringUnitProgramLinks} from '../../js/src/javascript-string-unit-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-string-units.lino',import.meta.url),createJavaScriptStringUnitProgramLinks);
