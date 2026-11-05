import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/whole-parameter-defaults.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/whole-parameter-default-program'));
