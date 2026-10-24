// Future complete paired producer. Never execute during the whole draft.
import { produceJavaScriptProgramOracle } from '../javascript-program/oracle-source.mjs';
import { createReferenceArrayProjectionProgramLinks } from '../../js/src/reference-array-projection-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/reference-array-projections.lino', import.meta.url), createReferenceArrayProjectionProgramLinks);
