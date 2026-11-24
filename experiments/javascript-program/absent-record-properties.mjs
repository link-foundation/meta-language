import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/absent-record-properties.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/absence-only-program'));
