// Future complete paired producer. Never execute during the whole draft.
import { produceJavaScriptProgramOracle } from '../javascript-program/oracle-source.mjs';
import { createJavaScriptIdentifierProgramLinks } from '../../js/src/javascript-identifier-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-escaped-identifiers.lino', import.meta.url), createJavaScriptIdentifierProgramLinks);
