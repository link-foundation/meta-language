import { produceJavaScriptProgramOracle } from '../javascript-program/oracle-source.mjs';
import { createSourceIdentifierProgramLinks } from '../../js/src/source-identifier-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/source-identifiers.lino', import.meta.url), createSourceIdentifierProgramLinks);
