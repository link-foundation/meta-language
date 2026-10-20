import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {generateRootTypeOptionHistory} from '../scripts/generate-root-type-option-history.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';

const root=new URL('../../',import.meta.url);
test('the native root-type compiler correction preserves every whole original source assertion and graph fixture',()=>{
  const history=generateRootTypeOptionHistory({check:true}),source=history.sources[0];
  verifyGitArtifactCertificate(source.certificate,Buffer.from(source.sourceText),{revision:history.revision,path:source.file});
  let current=readFileSync(new URL(source.file,root),'utf8');
  for(const action of [...history.actions].reverse()){
    assert.equal(current.split(action.to).length,2);current=current.replace(action.to,action.from);
  }
  assert.equal(current,source.sourceText);
  assert.equal(history.actions.length,5);assert.equal(history.observedCompilerDiagnostics.error,'E0308');
  assert.equal(history.observedCompilerDiagnostics.expected,'Option<LinkType>');
  assert.equal(history.observedCompilerDiagnostics.actual,'LinkType');
  assert.ok(history.accessor.declaration.includes('-> Option<LinkType>'));
  assert.equal(history.observedLintDiagnostics.correctedBinding,'type_designations');
});

test('neither unrelated source edits nor loss of a root-type assertion can satisfy the exact retained correction',()=>{
  const history=generateRootTypeOptionHistory({check:true}),source=history.sources[0];
  assert.throws(()=>verifyGitArtifactCertificate(source.certificate,Buffer.from(source.sourceText.replace('assert_eq!','assert_ne!')),{revision:history.revision,path:source.file}));
  const corrected=readFileSync(new URL(source.file,root),'utf8');
  assert.ok(corrected.includes('Some(root_type_kind(declaration["type"].as_str().unwrap()))'));
  assert.ok(corrected.includes('link.metadata().link_type() == Some(LinkType::Type)'));
  assert.ok(corrected.includes('.with_link_type(root_type_kind('),'the constructor still takes the original nonoptional type');
  assert.equal((corrected.match(/\bassert(?:_eq|_ne)?!/gu)??[]).length,(source.sourceText.match(/\bassert(?:_eq|_ne)?!/gu)??[]).length);
});
