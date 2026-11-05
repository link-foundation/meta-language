import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/named-default-callables.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/named-default-callable-program'));
