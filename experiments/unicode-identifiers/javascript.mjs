// Future complete paired producer. Never execute during the whole draft.
import { produceJavaScriptProgramOracle } from '../javascript-program/oracle-source.mjs';
import { createUnicodeScalarProgramLinks } from '../../js/src/unicode-scalar-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/unicode-javascript-identifiers.lino', import.meta.url), createUnicodeScalarProgramLinks);
