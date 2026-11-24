import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptRecordParameterProgramLinks} from '../../js/src/javascript-record-parameter-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/object-binding-declarations.lino',import.meta.url),createJavaScriptRecordParameterProgramLinks);
