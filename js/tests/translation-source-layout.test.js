import assert from 'node:assert/strict';
import test from 'node:test';
import {selfTranslate} from '../src/self-translation.js';
import {requireSelfTranslationLayoutEnvelope} from '../src/self-translation-layout-rules.js';
import {generateSelfTranslationLayoutRules} from '../scripts/self-translation-layout-generation.mjs';

test('source envelopes preserve every original gap prefix suffix comment and Unicode byte through unedited translation',()=>{
  generateSelfTranslationLayoutRules({check:true});
  for(let bits=0;bits<16;bits++)assert.equal(requireSelfTranslationLayoutEnvelope(Boolean(bits&1),Boolean(bits&2),Boolean(bits&4),Boolean(bits&8)),bits!==0);
  const sources=['const a=1;const b=2;','  const a=1;\tconst b=2; \n  ','\n\nconst a=1;\n \t\nconst b=2;\t','/*😀*/const a=1;/*é*/const b=2;',' \t\n','', '/** @returns {number} */function value(){return 1;}console.log(value());'];
  for(const source of sources){
    const translated=selfTranslate(source,'JavaScript','Rust');
    assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source,JSON.stringify(source));
  }
});

test('edited target bodies cannot restore obsolete source through the expanded layout envelope',async()=>{
  const source='  /** @returns {number} */function value(){return 1;}\t';
  const translated=selfTranslate(source,'JavaScript','Rust');assert.match(translated.code,/ envelope-sha256=/u);
  const index=translated.code.lastIndexOf('1f64');assert.ok(index>=0);
  const changed=translated.code.slice(0,index)+'2f64'+translated.code.slice(index+4);
  const restored=selfTranslate(changed,'Rust','JavaScript').code;
  assert.notEqual(restored,source);
  const imported=await import('data:text/javascript;base64,'+Buffer.from(restored).toString('base64'));
  assert.equal(imported.value(),2);
});
