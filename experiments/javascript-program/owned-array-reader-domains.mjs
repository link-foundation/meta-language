import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createSourceDefinitionProgramLinks} from '../../js/src/source-definition-catalog.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/owned-array-reader-domains.lino',import.meta.url),()=>createSourceDefinitionProgramLinks('meta-language/source-program/owned-array-reader-domain-program'));
