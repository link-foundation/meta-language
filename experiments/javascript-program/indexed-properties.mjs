import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptLiteralPropertyProgramLinks} from '../../js/src/javascript-literal-property-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-indexed-properties.lino',import.meta.url),createJavaScriptLiteralPropertyProgramLinks);
