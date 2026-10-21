import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url),revision='6ca7b4a02a524e096d4f34fb7f9df39e16263328';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const matrix=count=>Array.from({length:count},(_,index)=>`          - {group: translation, partition: ${index+1}, shards: ${count}}\n`).join('');
const nativeStep='      - name: Validate TypeScript graph Rust execution\n        if: ${{ !cancelled() }}\n        run: node experiments/typescript-typed-graph/run-native.mjs --results-dir issue-195-results\n\n';
const artifactScopes=['fresh-merge-ubuntu-latest-all-${{ matrix.physical_shard }}',
  'test-${{ matrix.os }}-${{ matrix.logical_suite || matrix.suite }}-${{ matrix.physical_shard || 1 }}',
  'coverage-ubuntu-latest-${{ matrix.logical_suite || matrix.suite }}-${{ matrix.physical_shard || 1 }}'];
const artifactStep=scope=>'      - name: Retain TypeScript graph native observations\n'
  +"        if: ${{ !cancelled() && hashFiles('issue-195-artifacts/typescript-typed-graph-native/**') != '' }}\n"
  +'        uses: actions/upload-artifact@v7\n        with:\n          name: typescript-typed-graph-'+scope+'-${{ github.sha }}\n'
  +'          path: issue-195-artifacts/typescript-typed-graph-native/\n          if-no-files-found: error\n          retention-days: 30\n\n';

/** Keep complete original workflows and an exact inverse of each additive edit. */
export function generateTranslationWorkflowHistory({check=false}={}) {
  const workflows=['.github/workflows/js.yml','.github/workflows/ci.yml','.github/workflows/rust.yml'].map(file=>{
    const captured=captureGitArtifactCertificate(revision,file),sourceText=captured.blob.toString('utf8');
    verifyGitArtifactCertificate(captured.certificate,captured.blob,{revision,path:file});
    let expected;
    if(file.endsWith('/js.yml')){
      assert.equal(sourceText.split(matrix(3)).length,2);expected=sourceText.replace(matrix(3),matrix(6));
      const changes=[
        ['          --commit "$GITHUB_SHA" --shard "$TRANSLATION_SHARD"\n','          --commit "$GITHUB_SHA" --shard "$TRANSLATION_SHARD" --partitions "${{ matrix.shards }}"\n'],
        ['          pattern: translation-test-feedback-${{ github.sha }}-*-of-3\n','          pattern: translation-test-feedback-${{ github.sha }}-*-of-6\n'],
        ['          node js/scripts/translation-test-feedback.mjs merge \\\n','          node js/scripts/translation-test-feedback.mjs merge \\\n            --partitions 6 \\\n'],
      ];
      for(const[before,after]of changes){assert.equal(expected.split(before).length,2);expected=expected.replace(before,after);}
    }else if(file.endsWith('/ci.yml')){
      const start=sourceText.indexOf('  native-rust:\n'),end=sourceText.indexOf('\n  native-translations:',start);
      assert.ok(start>=0&&end>start);
      const region=sourceText.slice(start,end),anchor='          --stage "$TARGET"\n\n';
      assert.equal(region.split(anchor).length,2);
      expected=sourceText.slice(0,start)+region.replace(anchor,anchor+nativeStep)+sourceText.slice(end);
    }else{
      let index=0;
      expected=sourceText.replace(/^      - name: Retain authenticated (?:fresh merge|native) partition\n/gmu,
        value=>artifactStep(artifactScopes[index++])+value);
      assert.equal(index,3);
    }
    const current=restoreEvidenceTranslationSourceHistory(file,restoreCurrentCiRepairSource(readFileSync(new URL(file,root),'utf8'),file));assert.equal(current,expected,'only the declared complete scheduling changes are admitted');
    let start=0,end=sourceText.length,finish=current.length;
    while(start<end&&start<finish&&sourceText[start]===current[start])start++;
    while(end>start&&finish>start&&sourceText[end-1]===current[finish-1]){end--;finish--;}
    const edit={offsetUnit:'UTF-8 bytes',start:Buffer.byteLength(current.slice(0,start)),end:Buffer.byteLength(current.slice(0,finish)),
      removed:sourceText.slice(start,end),inserted:current.slice(start,finish)};
    const bytes=Buffer.from(current),inverse=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.removed),bytes.subarray(edit.end)]);
    assert.ok(inverse.equals(captured.blob));
    return{file,commit:revision,...pin(captured.blob),sourceText,certificate:captured.certificate,current:pin(current),edit};
  });
  const history={schemaVersion:1,revision,workflows,oldTranslationPartitions:3,newTranslationPartitions:6,
    translationDeadlineMinutes:15,
    boundary:'Full workflow source preservation and exact scheduling inverses only. Every previous command, test file, assertion, failure predicate, artifact root and deadline remains required. The additional native TypeScript control runs after JavaScript through the existing Rust stage; separate uploads retain native package observations without disturbing sealed partition layouts. Native behavior still needs actual CI execution.'};
  const file=new URL('parity/self-translation/translation-workflow-history.lino',root);
  if(check)checkProjectData(file,history);else writeProjectData(file,history);
  const helper=new URL('js/tests/support/translation-workflow-history.js',root),source=readFileSync(helper,'utf8');
  const expected=source.replace(/const historyHash='(?:[a-f0-9]{64}|PENDING)';/u,"const historyHash='"+pin(readFileSync(file)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helper,expected);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateTranslationWorkflowHistory({check:process.argv.includes('--check')});
  console.log('Preserved three complete original workflows, every test and the fifteen-minute translation deadline.');
}

import {restoreEvidenceTranslationSourceHistory} from './evidence-translation-source-history.mjs';

import {restoreCurrentCiRepairSource} from './current-ci-repair-source-history.mjs';
