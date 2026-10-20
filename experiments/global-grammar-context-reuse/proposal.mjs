import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
const path=new URL('../../parity/self-translation/global-grammar-context-reuse.lino',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

/** Reconstruct the complete reviewed proposed source; never rewrite a producer. */
export function readGlobalGrammarContextProposal(file) {
  const packet=readProjectData(path),rows=packet.sources.filter(row=>row.file===file);assert.equal(rows.length,1);
  const row=rows[0];verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:file});
  assert.deepEqual(pin(row.sourceText),{bytes:row.bytes,sha256:row.sha256});
  assert.equal(readFileSync(new URL('../../'+file,import.meta.url),'utf8'),row.sourceText,'unchanged explicit predecessor');
  let source=row.sourceText;
  for(const action of row.actions){assert.equal(source.split(action.before).length,2);source=source.replace(action.before,action.after);}
  assert.deepEqual(pin(source),row.proposed);
  let inverse=source;for(const action of row.actions.toReversed()){assert.equal(inverse.split(action.after).length,2);inverse=inverse.replace(action.after,action.before);}
  assert.equal(inverse,row.sourceText);
  return{file,source,proposed:row.proposed,original:{bytes:row.bytes,sha256:row.sha256}};
}
