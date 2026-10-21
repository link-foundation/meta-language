// Package one complete source definition, checked through the normal frontend.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {readCallableCapturedBindings} from '../src/translation/callable-source.js';
import {checkProjectData,readProjectData,writeProjectData} from './project-data.mjs';

export function readStringRelativeCharacterTemplateGeneration() {
  const sourcePath='js/src/translation/string-relative-character-rules.js';
  const source=readFileSync(new URL('../src/translation/string-relative-character-rules.js',import.meta.url),'utf8');
  const surface=parseJavaScript(source,{referenceSemantics:false});
  const checked=checkProgram(surface);
  assert.equal(surface.items.length,1);assert.equal(checked.items.length,1);assert.equal(checked.declarations.size,1);
  const originalDeclaration=surface.items[0],checkedDeclaration=checked.items[0];
  assert.equal(originalDeclaration.k,'fn');assert.equal(originalDeclaration.name,'readStringRelativeCharacter');
  assert.deepEqual(originalDeclaration.params.map(parameter=>parameter.type.kind),['string','float']);
  assert.deepEqual(originalDeclaration.ret,{kind:'option',element:{kind:'string'},absence:'undefined'});
  assert.equal(readCallableCapturedBindings(checkedDeclaration.body,checkedDeclaration.params.map(parameter=>parameter.name)).size,0);
  const template={k:'lambda',params:structuredClone(originalDeclaration.params),body:structuredClone(originalDeclaration.body),
    ret:structuredClone(originalDeclaration.ret),span:originalDeclaration.span};
  return{schemaVersion:1,model:'portable UTF16 relative character lambda 1',sourcePath,source,
    sourceSha256:createHash('sha256').update(source).digest('hex'),originalDeclaration:structuredClone(originalDeclaration),
    checkedDeclaration:structuredClone(checkedDeclaration),template,
    languageVersions:{language:'ECMAScript',minimum:'2022',maximumExclusive:'2027'},
    inputDomain:'Primitive UTF-16 strings and evaluated Number positions, including represented nullable numeric conversion; an omitted position is zero. Other ToNumber conversions, custom builtins and extra method arguments remain explicit obligations.',
    evaluation:'Apply the complete closed lambda to the separately checked receiver and position once in source order; lexical names inside the lambda cannot capture actual operands.',
    result:'One original UTF-16 code unit at a nonnegative index or at length plus a negative integer position; out-of-range positions return undefined rather than an empty string.',
    reverseBoundary:'The complete checked graph uses existing optional values, functions, arithmetic, comparisons and UTF-16 slice operations. A Rust helper name alone supplies no source interpretation.'};
}

export function generateStringRelativeCharacterTemplate(check=false) {
  const policy=readStringRelativeCharacterTemplateGeneration();
  for(const file of [new URL('../src/translation/data/string-relative-character-template.lino',import.meta.url),
    new URL('../../rust/src/data/string-relative-character-template.lino',import.meta.url)]){
    if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
    assert.deepEqual(readProjectData(file),policy);
  }
  const units=[[],[65,66,67],[0,65535],[65,0xd83d,0xde00,66],[0xd800,120,0xdfff],[0xd800],[0xdfff]];
  const positions=['-Infinity','-9007199254740992','-5','-4','-3','-2','-1.9','-1','-0.9','-5e-324','-0','0','5e-324','0.9','1','1.9','2','3','4','9007199254740992','Infinity','NaN'];
  const fixture={schemaVersion:1,model:policy.model,languageVersions:policy.languageVersions,
    source:'/** @param {string} text @param {number} position @returns {string|undefined} */function relativeCharacter(text,position){return text.at(position);}',
    units,positions,cases:units.flatMap((input,text)=>positions.map((spelling,position)=>{
      const value=String.fromCharCode(...input).at(Number(spelling));
      return{text,position,undefined:value===undefined,units:value===undefined?[]:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))};
    })),
    boundary:'Independent Node primitive String.prototype.at observations on actual UTF-16 inputs. This table supplies expected values; it does not claim that native code executed.'};
  const fixtureFile=new URL('../../parity/fixtures/string-relative-character-observations.lino',import.meta.url);
  if(check)checkProjectData(fixtureFile,fixture);else writeProjectData(fixtureFile,fixture);
  return policy;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  generateStringRelativeCharacterTemplate(process.argv.includes('--check'));
  console.log('UTF-16 relative character template: one complete checked declaration; two ordered operands');
}
