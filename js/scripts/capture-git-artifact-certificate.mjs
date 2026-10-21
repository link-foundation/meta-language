// Extract exact Git objects automatically; normal verification needs no Git DB.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

export function captureGitArtifactCertificate(revision,path,{repository=fileURLToPath(new URL('../../',import.meta.url))}={}) {
  assert.ok(typeof revision==='string'&&revision.length===40&&/^[0-9a-f]{40}$/u.test(revision));
  assert.equal(typeof path,'string');const components=path.split('/');
  assert.ok(components.length>0&&components.length<=64&&components.every(name=>name&&!['.','..'].includes(name)&&!/[\0\\]/u.test(name)));
  const git=(kind,id)=>execFileSync('git',['--no-replace-objects','cat-file',kind,id],{cwd:repository,maxBuffer:64*1024*1024,timeout:15000});
  const commit=git('commit',revision),commitText=commit.toString('utf8');assert.deepEqual(Buffer.from(commitText),commit);
  const root=/^tree ([0-9a-f]{40})\n/u.exec(commitText);assert.ok(root);
  const trees=[];let objectId=root[1];
  for(const component of components){
    const bytes=git('tree',objectId),entries=[];let at=0;
    while(at<bytes.length){
      const space=bytes.indexOf(32,at),zero=bytes.indexOf(0,space+1);
      assert.ok(space>at&&zero>space+1&&zero+21<=bytes.length);
      const mode=bytes.subarray(at,space).toString('ascii'),nameBytes=bytes.subarray(space+1,zero),name=nameBytes.toString('utf8');
      assert.deepEqual(Buffer.from(name),nameBytes,'original tree names must remain lossless');
      entries.push({mode,name,objectId:bytes.subarray(zero+1,zero+21).toString('hex')});at=zero+21;
    }
    assert.equal(at,bytes.length);trees.push({objectId,entries});
    const selected=entries.find(entry=>entry.name===component);assert.ok(selected,'the pinned Git path exists');objectId=selected.objectId;
  }
  const blob=git('blob',objectId);
  const certificate={schemaVersion:1,objectFormat:'sha1',revision,path,commitText,trees,
    blob:{objectId,bytes:blob.length,sha256:createHash('sha256').update(blob).digest('hex')}};
  assert.deepEqual(verifyGitArtifactCertificate(certificate,blob,{revision,path}),blob);
  return{certificate,blob};
}

if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2),options={};
  for(let at=0;at<args.length;at++){
    const key=args[at];assert.ok(['--revision','--path','--certificate','--blob','--check'].includes(key));assert.equal(options[key],undefined);
    options[key]=key==='--check'?true:args[++at];assert.ok(options[key]);
  }
  for(const key of ['--revision','--path','--certificate','--blob'])assert.equal(typeof options[key],'string');
  assert.notEqual(resolve(options['--certificate']),resolve(options['--blob']));
  assert.ok(options['--certificate'].endsWith('.lino')&&options['--blob'].endsWith('.lino'),'linked proof and original linked artifact');
  const {certificate,blob}=captureGitArtifactCertificate(options['--revision'],options['--path']);
  if(options['--check']){checkProjectData(options['--certificate'],certificate);assert.deepEqual(readFileSync(options['--blob']),blob);}
  else{writeProjectData(options['--certificate'],certificate);writeFileSync(options['--blob'],blob);}
  console.log('Authenticated original Git commit, '+certificate.trees.length+' complete trees and '+blob.length+' exact artifact bytes.');
}
