// Future complete paired producer. Never execute during the whole draft.
import { produceJavaScriptProgramOracle } from '../javascript-program/oracle-source.mjs';
import { createJavaScriptPropertyCallProgramLinks } from '../../js/src/javascript-property-call-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-property-calls.lino', import.meta.url), createJavaScriptPropertyCallProgramLinks);
