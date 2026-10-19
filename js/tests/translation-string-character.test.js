import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import {format} from 'node:util';
import test from 'node:test';
import {readStringCharacter} from '../src/translation/string-character-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {selfTranslate} from '../src/self-translation.js';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {readStringCodeUnitTemplateGeneration} from '../scripts/generate-string-code-unit-template.mjs';

const source=`/** @param {string} text @param {number} position @returns {string} */
function character(text,position){return text.charAt(position);}
/** @param {string} text @param {number|null|undefined} position @returns {string} */
function nullableCharacter(text,position){return text.charAt(position);}`;
const texts=['','ABC','\u0000\uffff','A😀B','\ud800x\udfff','\ud800','\udfff'];
const positions=[-Infinity,-9007199254740992,-2,-1,-0.9,-Number.MIN_VALUE,-0,0,Number.MIN_VALUE,0.9,1,1.9,2,3,4,9007199254740992,Infinity,NaN];

test('portable character selection preserves individual UTF16 units numeric index normalization and empty results',()=>{
  const program=checkProgram(parseJavaScript(source));
  const compiled=runInNewContext(emitJavaScript(program).text+';({character,nullableCharacter});');
  for(const text of texts)for(const position of positions){
    const expected=text.charAt(position);
    assert.equal(readStringCharacter(text,position),expected);
    assert.equal(compiled.character(text,position),expected);
    assert.equal(compiled.nullableCharacter(text,position),expected);
  }
  for(const position of [null,undefined])assert.equal(compiled.nullableCharacter('ABC',position),'A');
  assert.equal(compiled.character('😀',0),'\ud83d');assert.equal(compiled.character('😀',1),'\ude00');
  assert.equal(compiled.character('ABC',3),'');
});

test('character lowering preserves defaults operand evaluation order lexical ownership and explicit conversion boundaries',()=>{
  const ordered=`/** @returns {string} */function receiver(){console.log('receiver');return 'ABC';}
/** @returns {number} */function position(){console.log('position');return 1.9;}
console.log(receiver().charAt(position()));console.log(receiver().charAt());
console.log('ABC'.charAt(undefined),'ABC'.charAt(null),'ABC'.charAt(NaN));`;
  const represented=ordered.replace("console.log('ABC'.charAt(undefined),", "console.log('%s %s %s','ABC'.charAt(undefined),");
  for(const text of [ordered,represented,emitJavaScript(checkProgram(parseJavaScript(represented))).text]){
    const lines=[];runInNewContext(text,{console:{log:(...values)=>lines.push(format(...values))}});
    assert.deepEqual(lines,['receiver','position','B','receiver','A','A A A']);
  }
  const collision=`/** @param {string} text @param {number} position @param {number} Math @param {number} Number @returns {string} */
function collision(text,position,Math,Number){const readStringCharacter=999;return text.charAt(position);}`;
  const compiled=runInNewContext(emitJavaScript(checkProgram(parseJavaScript(collision))).text+';collision;');
  assert.equal(compiled('ABC',1,8,9),'B');
  for(const position of ['true','false',"'1'",'{}','[1]'])assert.throws(()=>checkProgram(parseJavaScript("console.log('ABC'.charAt("+position+'));')));
  assert.throws(()=>checkProgram(parseJavaScript("console.log('ABC'.charAt(0,1));")),/expects 0 to 1 arguments/u);
});

test('character selection rechecks after complete text and binary Links reload without a special graph operation',()=>{
  const packet=createTypedProgramLinks(source,'JavaScript');
  const native=emitRust(checkProgram(parseJavaScript(source))).text;
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
    const restored=decodeTypedProgramLinks(network,packet.root);recheckLinkedTypedProgram(restored,{wholeProgram:true});
    assert.equal(JSON.stringify(restored.items).includes('"op":"charAt"'),false);
    const javascript=emitTypedProgramLinks(network,packet.root,'JavaScript').code;
    assert.equal(emitTypedProgramLinks(network,packet.root,'Rust').code,native);
    const character=runInNewContext(javascript+';character;');
    for(const text of texts)for(const position of positions)assert.equal(character(text,position),text.charAt(position));
  }
});

test('the packaged character extension preserves all preceding code unit policy bytes and complete source round trips',()=>{
  const policy=readStringCodeUnitTemplateGeneration();
  assert.equal(policy.character.source,readFileSync(new URL('../src/translation/string-character-rules.js',import.meta.url),'utf8'));
  assert.equal(policy.character.originalDeclaration.name,'readStringCharacter');
  const history=readProjectData(new URL('../../parity/naming/string-character-source-history.lino',import.meta.url));
  const old=history.sources.find(row=>row.file==='js/src/translation/data/string-code-unit-template.lino');
  const restored=structuredClone(policy);delete restored.character;
  assert.deepEqual(restored,readStructuredDataLino(old.sourceText));
  assert.equal(serializeProjectData(restored,old.file),old.sourceText);
  for(const file of ['js/src/translation/data/string-code-unit-template.lino','rust/src/data/string-code-unit-template.lino'])assert.deepEqual(readProjectData(new URL('../../'+file,import.meta.url)),policy);
  const translated=selfTranslate(source,'JavaScript','Rust');
  assert.equal(translated.items.filter(row=>row.status==='carried').length,0);
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
});
