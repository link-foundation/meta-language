// Future complete paired producer. Never execute during the whole draft.
import { produceRustNominalOracle } from '../rust-nominal/oracle-source.mjs';
import { createUnicodeScalarProgramLinks } from '../../js/src/unicode-scalar-program.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-unicode-lifetimes.lino', import.meta.url), createUnicodeScalarProgramLinks);
