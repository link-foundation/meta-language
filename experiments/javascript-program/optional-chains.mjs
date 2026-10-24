import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createJavaScriptOptionalChainProgramLinks} from '../../js/src/javascript-optional-chain-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-optional-chains.lino',import.meta.url),createJavaScriptOptionalChainProgramLinks);
