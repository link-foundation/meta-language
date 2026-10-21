// The existing code-unit generator packages this complete sibling template.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {readCallableCapturedBindings} from '../src/translation/callable-source.js';

export function readStringCharacterTemplateGeneration() {
  const sourcePath='js/src/translation/string-character-rules.js';
  const source=readFileSync(new URL('../src/translation/string-character-rules.js',import.meta.url),'utf8');
  const surface=parseJavaScript(source,{referenceSemantics:false});
  const checked=checkProgram(surface);
  assert.equal(surface.items.length,1);assert.equal(checked.items.length,1);assert.equal(checked.declarations.size,1);
  const originalDeclaration=surface.items[0],checkedDeclaration=checked.items[0];
  assert.equal(originalDeclaration.k,'fn');assert.equal(originalDeclaration.name,'readStringCharacter');
  assert.deepEqual(originalDeclaration.params.map(parameter=>parameter.type.kind),['string','float']);
  assert.equal(originalDeclaration.ret.kind,'string');
  assert.equal(readCallableCapturedBindings(checkedDeclaration.body,checkedDeclaration.params.map(parameter=>parameter.name)).size,0);
  const template={k:'lambda',params:structuredClone(originalDeclaration.params),body:structuredClone(originalDeclaration.body),
    ret:structuredClone(originalDeclaration.ret),span:originalDeclaration.span};
  return{schemaVersion:1,model:'portable UTF16 character lambda 1',sourcePath,source,
    sourceSha256:createHash('sha256').update(source).digest('hex'),originalDeclaration:structuredClone(originalDeclaration),
    checkedDeclaration:structuredClone(checkedDeclaration),template,
    inputDomain:'Primitive UTF-16 string and evaluated Number index, including represented nullable numeric conversion; omitted position is zero. Other ToNumber conversions remain explicit obligations.',
    evaluation:'Instantiate the complete closed source lambda and apply separately evaluated receiver and position once, preserving their original order and caller span.'};
}
