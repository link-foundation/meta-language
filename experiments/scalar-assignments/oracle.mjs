import { produceJavaScriptProgramOracle } from '../javascript-program/oracle-source.mjs';
import { createScalarAssignmentProgramLinks } from '../../js/src/scalar-assignment-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/scalar-assignments.lino',import.meta.url),createScalarAssignmentProgramLinks);
