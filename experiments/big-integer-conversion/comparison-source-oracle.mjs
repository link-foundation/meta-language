import {produceJavaScriptProgramOracle} from '../javascript-program/oracle-source.mjs';
import {createJavaScriptComparisonProgramLinks} from '../../js/src/javascript-comparison-program.js';
produceJavaScriptProgramOracle(new URL('../../parity/fixtures/primitive-comparisons.lino',import.meta.url),createJavaScriptComparisonProgramLinks);
