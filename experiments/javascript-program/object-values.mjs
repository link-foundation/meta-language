import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptRecordOrderProgramLinks} from '../../js/src/javascript-record-order-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-object-values.lino',import.meta.url),createJavaScriptRecordOrderProgramLinks);
