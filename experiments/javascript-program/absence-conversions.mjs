import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/absence-conversions.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/absence-conversion-program'));
