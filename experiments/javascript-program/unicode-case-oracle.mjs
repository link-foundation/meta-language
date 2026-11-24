import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createUnicodeCaseProgramLinks} from '../../js/src/unicode-case-program.js';
if(process.versions.node.split('.')[0]!=='24'||process.versions.unicode!=='16.0')throw new RangeError('uncovered Unicode casing oracle '+process.versions.node+'/'+process.versions.unicode);
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/unicode-case-values.lino',import.meta.url),createUnicodeCaseProgramLinks);
