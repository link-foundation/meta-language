// Derive one complete source lambda, retaining its actual declaration and types.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {runInNewContext} from 'node:vm';
import {format} from 'node:util';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {readCallableCapturedBindings} from '../src/translation/callable-source.js';
import {checkProjectData,readProjectData,writeProjectData} from './project-data.mjs';

export function readArrayRelativeElementTemplateGeneration() {
  const sourcePath='js/src/translation/array-relative-element-rules.js';
  const source=readFileSync(new URL('../src/translation/array-relative-element-rules.js',import.meta.url),'utf8');
  const surface=parseJavaScript(source,{referenceSemantics:false}),checked=checkProgram(surface);
  assert.equal(surface.items.length,1);assert.equal(checked.items.length,1);assert.equal(checked.declarations.size,1);
  const originalDeclaration=surface.items[0],checkedDeclaration=checked.items[0];
  assert.equal(originalDeclaration.k,'fn');assert.equal(originalDeclaration.name,'readArrayRelativeElement');
  assert.equal(originalDeclaration.params[0].type.kind,'array');assert.equal(originalDeclaration.params[0].type.element.kind,'float');
  assert.equal(originalDeclaration.params[1].type.kind,'float');
  assert.deepEqual(originalDeclaration.ret,{kind:'option',element:{kind:'float'},absence:'undefined'});
  assert.equal(readCallableCapturedBindings(checkedDeclaration.body,checkedDeclaration.params.map(parameter=>parameter.name)).size,0);
  const template={k:'lambda',params:structuredClone(originalDeclaration.params),body:structuredClone(originalDeclaration.body),ret:structuredClone(originalDeclaration.ret),span:originalDeclaration.span};
  return{schemaVersion:1,model:'portable dense array relative element lambda 1',sourcePath,source,
    sourceSha256:createHash('sha256').update(source).digest('hex'),originalDeclaration:structuredClone(originalDeclaration),checkedDeclaration:structuredClone(checkedDeclaration),template,
    languageVersions:{language:'ECMAScript',minimum:'2022',maximumExclusive:'2027'},
    specialization:{receiverParameter:0,positionParameter:1,receiverKind:'array',resultKind:'option',absence:'undefined',
      boundary:'Specialize only the actual receiver parameter and returned element type, then recheck the complete closed lambda with the actual typed program. Array mutation, holes, inherited properties and accessors require their own shared reference model.'},
    evaluation:'Apply the complete closed lambda to the independently checked receiver and numeric position exactly once in source order.',
    boundary:'A source kernel and typed lambda template; public method integration, native execution and full language coverage require their own checks.'};
}

export function generateArrayRelativeElementTemplate(check=false) {
  const policy=readArrayRelativeElementTemplateGeneration();
  for(const file of [new URL('../src/translation/data/array-relative-element-template.lino',import.meta.url),new URL('../../rust/src/data/array-relative-element-template.lino',import.meta.url)]){
    if(check)checkProjectData(file,policy);else writeProjectData(file,policy);
    assert.deepEqual(readProjectData(file),policy);
  }
  const positions=['-Infinity','-9007199254740992','-5','-4','-3','-2','-1.9','-1','-0.9','-5e-324','-0','0','5e-324','0.9','1','1.9','2','3','4','9007199254740992','Infinity','NaN'];
  const declaration='/** @param {number[]} values @param {number} position @returns {number|undefined} */function relativeElement(values,position){return values.at(position);}' +
    '/** @param {string[]} values @param {number} position @returns {string|undefined} */function relativeText(values,position){return values.at(position);}' +
    '/** @param {boolean[]} values @param {number} position @returns {boolean|undefined} */function relativeBoolean(values,position){return values.at(position);}';
  const numeric=declaration+['[]','[0]','[1,2,3]','[-0,NaN,Infinity,-Infinity]'].flatMap(values=>positions.map(position=>'console.log(relativeElement('+values+','+position+'));')).join('')+
    positions.map(position=>'console.log(relativeText(["A","😀"],'+position+'));console.log(relativeBoolean([true,false],'+position+'));').join('');
  const nullable='/** @param {(number|null|undefined)[]} values @param {number} position @returns {number|null|undefined} */function relativeNullable(values,position){return values.at(position);}' +
    positions.map(position=>'console.log(relativeNullable([null,undefined,7],'+position+'));').join('')+
    '/** @returns {number[]} */function receiver(){console.log("receiver");return [10,20,30];}/** @returns {number} */function position(){console.log("position");return -1.9;}' +
    'console.log(receiver().at(position()));console.log(receiver().at());console.log([1].at(undefined),[2].at(null),[3].at(NaN));';
  const programs=[['numeric and primitive elements',numeric],['nullable elements and ordered effects',nullable]].map(([id,source])=>{
    const lines=[];runInNewContext(source,{console:{log:(...values)=>lines.push(format(...values))}});
    return{id,source,expectedStdout:lines.join('\n')+'\n'};
  });
  const observations={schemaVersion:1,model:policy.model,languageVersions:policy.languageVersions,positions,programs,
    boundary:'Independent Node standard array observations. Rust compilation and execution of these complete programs occur only in CI; this fixture alone supplies no native execution evidence.'};
  const fixture=new URL('../../parity/fixtures/array-relative-element-observations.lino',import.meta.url);
  if(check)checkProjectData(fixture,observations);else writeProjectData(fixture,observations);
  return policy;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  generateArrayRelativeElementTemplate(process.argv.includes('--check'));
  console.log('Relative dense array element template: one complete checked declaration and two ordered operands.');
}
