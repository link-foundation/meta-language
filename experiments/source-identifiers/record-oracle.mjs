import { produceJavaScriptProgramOracle } from '../javascript-program/oracle-source.mjs';
import { createJavaScriptRecordParameterProgramLinks } from '../../js/src/javascript-record-parameter-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/source-identifiers.lino', import.meta.url), createJavaScriptRecordParameterProgramLinks);
