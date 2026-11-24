import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptRecordCallableParameterProgramLinks} from '../../js/src/record-callable-parameter-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/record-callable-parameters.lino',import.meta.url),createJavaScriptRecordCallableParameterProgramLinks);
