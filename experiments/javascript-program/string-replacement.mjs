import {produceJavaScriptProgramOracle} from './oracle-source.mjs';
import {createUtf16TextProgramLinks} from '../../js/src/utf16-text-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/string-replacement.lino',import.meta.url),createUtf16TextProgramLinks);
