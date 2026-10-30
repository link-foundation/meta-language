import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptNullishFieldProgramLinks} from '../../js/src/javascript-nullish-field-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-nullish-fields.lino',import.meta.url),createJavaScriptNullishFieldProgramLinks);
