// Future paired producer. Never execute during the whole requirements draft.
import{produceRustNominalOracle}from'../rust-nominal/oracle-source.mjs';
import{createRustInherentProgramLinks}from'../../js/src/rust-inherent-program.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-inherent-receiver-2.lino',import.meta.url),createRustInherentProgramLinks);
