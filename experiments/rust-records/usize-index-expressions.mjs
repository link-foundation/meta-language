// Future complete original/native producer; never execute while drafting.
import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-usize-index-expressions.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/rust-usize-index-expression-program'));
