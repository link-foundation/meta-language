import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createArrayBindingProgramLinks} from '../../js/src/array-binding-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/binding-defaults.lino',import.meta.url),createArrayBindingProgramLinks);
