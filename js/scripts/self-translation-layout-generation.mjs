import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';

export function generateSelfTranslationLayoutRules({check=false}={}) {
  const source=readFileSync(new URL('../src/self-translation-layout-rules.js',import.meta.url),'utf8');
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  assert.deepEqual([...program.declarations.keys()],['requireSelfTranslationLayoutEnvelope']);
  const emitted=emitRust(program,{arrayConsumption:true});assert.equal(emitted.preludes.length,0);
  return writeRecordRuleLibrary(emitted,'self_translation_layout_rules',{check,sourceStyles:source=>readBooleanParameterTupleStyle(source,'require_self_translation_layout_envelope')});
}

import {readBooleanParameterTupleStyle} from './boolean-parameter-tuple-style.mjs';
