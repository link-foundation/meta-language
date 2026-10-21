// Register sourced unabbreviated technical nouns; retain every preceding term
// and positive/negative fixture. Full WordNet and naming gates remain CI-only.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {readProjectData,serializeProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';

const root=new URL('../../',import.meta.url),revision='8f2f6c483ec32a3ffc17afec694463965dc7551d';
const args=process.argv.slice(2);assert.ok(args.length===0||args.length===1&&args[0]==='--check');
const check=args.includes('--check');
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const terms=[
  {word:'codebase',partsOfSpeech:['noun'],
    definition:'A source code repository, or related repositories sharing their original revision, maintained as one application source collection.',
    source:'Adam Wiggins, The Twelve-Factor App, I. Codebase: https://12factor.net/codebase'},
  {word:'frontend',partsOfSpeech:['noun'],
    definition:'The compiler component that reads and analyzes a source language before later optimization and target generation stages.',
    source:'LLVM Project, Clang Compiler User\u2019s Manual, terminology: https://clang.llvm.org/docs/UsersManual.html#terminology'},
];
const fixtures=[
  {name:'sourced codebase technical noun',expected:[],names:['repository.codebase']},
  {name:'sourced frontend technical noun',expected:[],names:['compiler.frontend']},
  {name:'codebase misspelling stays outside the vocabulary',expected:['vocabulary'],names:['codebaze']},
  {name:'frontend misspelling stays outside the vocabulary',expected:['vocabulary'],names:['frontent']},
];
const documents=[
  {path:'parity/naming/technical-vocabulary.lino',field:'terms',additions:terms},
  {path:'parity/naming/naming-fixtures.lino',field:'fixtures',additions:fixtures},
];
const receipts=[];
for(const {path,field,additions}of documents){
  const captured=captureGitArtifactCertificate(revision,path),original=captured.blob;
  verifyGitArtifactCertificate(captured.certificate,original,{revision,path});
  const document=readProjectData(new URL(path,root)),current=readFileSync(new URL(path,root),'utf8');
  const oldCount=field==='terms'?76:25;
  assert.ok(document[field].length===oldCount||document[field].length===oldCount+additions.length);
  if(document[field].length>oldCount)assert.deepEqual(document[field].splice(oldCount),additions);
  const previous=serializeProjectData(document,path);assert.deepEqual(Buffer.from(previous),original,'every original term, fixture, field, order and source byte survives');
  const keys=new Set(document[field].map(row=>field==='terms'?row.word:row.name));
  for(const row of additions){const key=field==='terms'?row.word:row.name;assert.ok(!keys.has(key));keys.add(key);}
  document[field].push(...additions);
  const expected=serializeProjectData(document,path);
  if(check){assert.equal(current,expected);checkProjectData(new URL(path,root),document);}else writeProjectData(new URL(path,root),document);
  receipts.push({path,field,previous:{...pin(original),certificate:captured.certificate},current:pin(expected),originalCount:oldCount,additions});
}
const receipt={schemaVersion:1,model:'sourced technical noun registration 1',revision,documents:receipts,
  boundary:'Only these two fully spelled technical nouns and four vocabulary fixtures append. Complete certified byte inverses retain all 76 prior terms and 25 fixtures. Canonical identities, bindings, WordNet 3.1, abbreviation rules and all naming gates remain unchanged. These source and serialization checks are not full naming, runtime or requirement evidence.'};
const target=new URL('parity/naming/ci-39644810104-vocabulary-additions.lino',root);
if(check)checkProjectData(target,receipt);else writeProjectData(target,receipt);
console.log('Registered two primary-source technical nouns with four vocabulary fixtures; every preceding byte remains reconstructible.');
