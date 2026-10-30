import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptPropertyCallProgramLinks} from '../../js/src/javascript-property-call-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/string-unit-calls.lino',import.meta.url),createJavaScriptPropertyCallProgramLinks);
