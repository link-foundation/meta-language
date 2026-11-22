// Extract exact Git objects automatically; normal verification needs no Git DB.
import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

const defaultRepository=fileURLToPath(new URL('../../',import.meta.url));
const readGitObject=(repository,kind,id)=>execFileSync('git',['--no-replace-objects','cat-file',kind,id],
  {cwd:repository,maxBuffer:64*1024*1024,timeout:15000});

export function captureGitArtifactCertificate(revision,path,{repository=defaultRepository}={}) {
  return captureCertificate(revision,path,(kind,id)=>readGitObject(repository,kind,id));
}

function readBlobBatch(repository,selections){
  const input=selections.map(({revision,path})=>revision+':'+path+'\n').join('');
  const output=execFileSync('git',['--no-replace-objects','cat-file','--batch'],
    {cwd:repository,input,maxBuffer:32*1024*1024,timeout:15000});
  let cursor=0;
  const blobs=selections.map(()=>{
    const end=output.indexOf(10,cursor);
    assert.ok(end>cursor&&end-cursor<80,'bounded Git blob frame header');
    const header=/^([0-9a-f]{40}) blob (0|[1-9][0-9]*)$/u.exec(output.toString('ascii',cursor,end));
    assert.ok(header,'every selected Git path resolves to a blob');
    const bytes=Number(header[2]),start=end+1,separator=start+bytes;
    assert.ok(Number.isSafeInteger(bytes)&&separator<output.length&&output[separator]===10);
    cursor=separator+1;
    return {objectId:header[1],blob:Buffer.from(output.subarray(start,separator))};
  });
  assert.equal(cursor,output.length,'every frame belongs to a requested original path');
  return blobs;
}

/** Bound each Git object batch and certify every selected path independently. */
export function* captureGitArtifactCertificates(selections,{repository=defaultRepository}={}) {
  const metadata=new Map(),maximumBytes=8*1024*1024;
  let cachedBytes=0;
  const readMetadata=(kind,id)=>{
    const key=kind+':'+id;
    const cached=metadata.get(key);
    if(cached)return cached;
    const bytes=readGitObject(repository,kind,id);
    if(cachedBytes+bytes.length<=maximumBytes){
      metadata.set(key,bytes);
      cachedBytes+=bytes.length;
    }
    return bytes;
  };
  function* captureBatch(selected){
    if(selected.length===1){
      const [{revision,path}]=selected;
      yield captureCertificate(revision,path,(kind,id)=>kind==='blob'?readGitObject(repository,kind,id):readMetadata(kind,id));
      return;
    }
    let blobs;
    try{blobs=readBlobBatch(repository,selected);}
    catch(error){
      if(error.code!=='ENOBUFS')throw error;
      const middle=Math.floor(selected.length/2);
      yield* captureBatch(selected.slice(0,middle));
      yield* captureBatch(selected.slice(middle));
      return;
    }
    for(const [index,{revision,path}]of selected.entries()){
      yield captureCertificate(revision,path,(kind,id)=>{
        if(kind!=='blob')return readMetadata(kind,id);
        const {objectId,blob}=blobs[index];
        assert.equal(objectId,id,'batch responses retain the certified path binding');
        return blob;
      });
    }
  }
  let selected=[];
  for(const {revision,path}of selections){
    certificatePath(revision,path);
    if(/[\r\n]/u.test(path)){
      if(selected.length)yield* captureBatch(selected);
      selected=[];
      yield* captureBatch([{revision,path}]);
    }else{
      selected.push({revision,path});
      if(selected.length===24){
        yield* captureBatch(selected);
        selected=[];
      }
    }
  }
  if(selected.length)yield* captureBatch(selected);
}

function certificatePath(revision,path) {
  assert.ok(typeof revision==='string'&&revision.length===40&&/^[0-9a-f]{40}$/u.test(revision));
  assert.equal(typeof path,'string');const components=path.split('/');
  assert.ok(components.length>0&&components.length<=64&&components.every(name=>name&&!['.','..'].includes(name)&&!/[\0\\]/u.test(name)));
  return components;
}

function captureCertificate(revision,path,git) {
  const components=certificatePath(revision,path);
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
