import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptArrayFormalParameterProgramLinks} from '../../js/src/array-formal-parameter-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/array-formal-parameters.lino',import.meta.url),createJavaScriptArrayFormalParameterProgramLinks);
