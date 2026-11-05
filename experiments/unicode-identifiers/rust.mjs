// Future producer only; never execute during drafting.
import { produceRustNominalOracle } from '../rust-nominal/oracle-source.mjs';
import { createUnicodeScalarProgramLinks } from '../../js/src/unicode-scalar-program.js';
produceRustNominalOracle(new URL('../../parity/fixtures/unicode-rust-identifiers.lino', import.meta.url), createUnicodeScalarProgramLinks);
