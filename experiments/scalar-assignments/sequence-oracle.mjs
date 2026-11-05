import {produceJavaScriptProgramOracle} from '../javascript-program/oracle-source.mjs';
import {createScalarAssignmentProgramLinks} from '../../js/src/scalar-assignment-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/sequence-expressions.lino',import.meta.url),createScalarAssignmentProgramLinks);
