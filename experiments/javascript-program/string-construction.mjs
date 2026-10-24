import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createStringConstructionProgramLinks} from '../../js/src/string-construction-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/string-construction.lino',import.meta.url),createStringConstructionProgramLinks);
