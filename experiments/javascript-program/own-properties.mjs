import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptOwnPropertyProgramLinks} from '../../js/src/javascript-own-property-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-own-properties.lino',import.meta.url),createJavaScriptOwnPropertyProgramLinks);
