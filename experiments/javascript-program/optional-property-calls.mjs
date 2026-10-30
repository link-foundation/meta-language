import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptOptionalCallProgramLinks} from '../../js/src/javascript-optional-call-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-optional-property-calls.lino',import.meta.url),createJavaScriptOptionalCallProgramLinks);
