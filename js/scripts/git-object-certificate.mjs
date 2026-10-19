// Offline membership proofs for immutable SHA-1 Git artifacts. Source bytes,
// complete tree entries and the actual commit retain their original encoding.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';

function checkCertificateRecord(value,keys) {
  assert.ok(value!==null&&typeof value==='object'&&!Array.isArray(value));
  assert.ok([Object.prototype,null].includes(Object.getPrototypeOf(value)));
  assert.deepEqual(Object.getOwnPropertySymbols(value),[]);
  assert.deepEqual(Object.getOwnPropertyNames(value).sort(),[...keys].sort());
  for(const key of keys){const descriptor=Object.getOwnPropertyDescriptor(value,key);assert.ok(descriptor.enumerable&&Object.hasOwn(descriptor,'value'));}
}

function gitObjectIdentity(kind,bytes) {
  return createHash('sha1').update(kind+' '+bytes.length+'\0').update(bytes).digest('hex');
}

function encodeCertificateTree(entries,limits) {
  assert.ok(Array.isArray(entries)&&entries.length<=limits.maxTreeEntries);
  assert.equal(Object.keys(entries).length,entries.length,'dense ordered Git tree entries');
  const names=new Set(),parts=[];let size=0;
  for(const entry of entries){
    checkCertificateRecord(entry,['mode','name','objectId']);
    assert.ok(['40000','100644','100755','120000','160000'].includes(entry.mode));
    assert.equal(typeof entry.name,'string');
    assert.ok(entry.name.length>0&&!['.','..'].includes(entry.name)&&!/[\0/]/u.test(entry.name));
    assert.equal(Buffer.from(entry.name,'utf8').toString('utf8'),entry.name,'lossless UTF-8 Git names');
    assert.ok(!names.has(entry.name),'unique complete tree names');names.add(entry.name);
    assert.ok(typeof entry.objectId==='string'&&entry.objectId.length===40&&/^[0-9a-f]{40}$/u.test(entry.objectId));
    const prefix=Buffer.from(entry.mode+' '+entry.name+'\0','utf8');
    size+=prefix.length+20;assert.ok(size<=limits.maxTreeBytes,'explicit complete tree byte bound');
    parts.push(prefix,Buffer.from(entry.objectId,'hex'));
  }
  return Buffer.concat(parts,size);
}

/** The caller supplies both trusted original anchors independently of the proof.
 * Schema 1 covers SHA-1 repositories with UTF-8 commit text and path names.
 * This operation reads no repository, filesystem, process or network state. */
export function verifyGitArtifactCertificate(certificate,blob,{revision,path,limits={}}) {
  const bounds={maxBlobBytes:64*1024*1024,maxTreeBytes:8*1024*1024,maxTreeEntries:100000,maxCommitBytes:4*1024*1024,maxPathDepth:64,...limits};
  assert.deepEqual(Object.keys(bounds).sort(),['maxBlobBytes','maxCommitBytes','maxPathDepth','maxTreeBytes','maxTreeEntries']);
  for(const value of Object.values(bounds))assert.ok(Number.isSafeInteger(value)&&value>0);
  assert.ok(typeof revision==='string'&&revision.length===40&&/^[0-9a-f]{40}$/u.test(revision),'trusted full original revision');
  assert.equal(typeof path,'string');const components=path.split('/');
  assert.ok(components.length>0&&components.length<=bounds.maxPathDepth);
  assert.ok(components.every(name=>name&&!['.','..'].includes(name)&&!/[\0\\]/u.test(name)),'exact safe original Git path');
  assert.equal(Buffer.from(path,'utf8').toString('utf8'),path);
  checkCertificateRecord(certificate,['schemaVersion','objectFormat','revision','path','commitText','trees','blob']);
  assert.equal(certificate.schemaVersion,1);assert.equal(certificate.objectFormat,'sha1');
  assert.equal(certificate.revision,revision);assert.equal(certificate.path,path);
  assert.equal(typeof certificate.commitText,'string');
  const commit=Buffer.from(certificate.commitText,'utf8');
  assert.equal(commit.toString('utf8'),certificate.commitText);assert.ok(commit.length<=bounds.maxCommitBytes);
  assert.equal(gitObjectIdentity('commit',commit),revision,'the complete original commit is the trust root');
  const root=/^tree ([0-9a-f]{40})\n/u.exec(certificate.commitText);assert.ok(root);
  assert.ok(Array.isArray(certificate.trees));assert.equal(certificate.trees.length,components.length);
  assert.equal(Object.keys(certificate.trees).length,components.length);
  let expected=root[1];
  for(let at=0;at<components.length;at++){
    const tree=certificate.trees[at];checkCertificateRecord(tree,['objectId','entries']);
    assert.equal(tree.objectId,expected);
    assert.equal(gitObjectIdentity('tree',encodeCertificateTree(tree.entries,bounds)),expected,'every complete original tree is authenticated');
    const entry=tree.entries.find(entry=>entry.name===components[at]);assert.ok(entry,'every original path component exists');
    if(at+1<components.length)assert.equal(entry.mode,'40000','intermediate owners are actual trees');
    else assert.ok(['100644','100755','120000'].includes(entry.mode),'the selected original object is a blob');
    expected=entry.objectId;
  }
  checkCertificateRecord(certificate.blob,['objectId','bytes','sha256']);
  assert.equal(certificate.blob.objectId,expected);
  assert.ok(Number.isSafeInteger(certificate.blob.bytes)&&certificate.blob.bytes>=0&&certificate.blob.bytes<=bounds.maxBlobBytes);
  assert.ok(Buffer.isBuffer(blob));assert.equal(blob.length,certificate.blob.bytes);
  assert.ok(typeof certificate.blob.sha256==='string'&&certificate.blob.sha256.length===64&&/^[0-9a-f]{64}$/u.test(certificate.blob.sha256));
  const bytes=Buffer.from(blob);
  assert.equal(gitObjectIdentity('blob',bytes),expected,'the exact original file belongs to the pinned commit and path');
  assert.equal(createHash('sha256').update(bytes).digest('hex'),certificate.blob.sha256);
  return bytes;
}
