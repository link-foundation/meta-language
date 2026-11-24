// Future paired producer. Never execute during the whole requirements draft.
import{produceRustNominalOracle}from'../rust-nominal/oracle-source.mjs';
import{createRustRecordProgramLinks}from'../../js/src/rust-record-program.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-pattern-conditions.lino',import.meta.url),createRustRecordProgramLinks);
