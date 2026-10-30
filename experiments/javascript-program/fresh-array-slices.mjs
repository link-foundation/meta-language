import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/javascript-fresh-array-slices.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/owned-array-allocation-program'));
