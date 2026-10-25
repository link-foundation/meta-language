import {produceRustNominalOracle} from '../rust-nominal/oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceRustNominalOracle(new URL('../../parity/fixtures/rust-match-guards.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/conditional-chain-program'));
