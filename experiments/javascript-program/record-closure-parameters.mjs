import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptRecordClosureParameterProgramLinks} from '../../js/src/record-closure-parameter-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/record-closure-parameters.lino',import.meta.url),createJavaScriptRecordClosureParameterProgramLinks);
