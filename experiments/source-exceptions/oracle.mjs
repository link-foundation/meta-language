import {produceJavaScriptProgramOracle} from '../javascript-program/oracle-source.mjs';
import {createSourceExceptionProgramLinks} from '../../js/src/source-exception-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/source-exceptions.lino',import.meta.url),createSourceExceptionProgramLinks);
