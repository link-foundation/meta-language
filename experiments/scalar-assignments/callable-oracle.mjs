import { produceJavaScriptProgramOracle } from '../javascript-program/oracle-source.mjs';
import { createCallableSourceProgramLinks } from '../../js/src/callable-source-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/scalar-assignments.lino',import.meta.url),createCallableSourceProgramLinks);
