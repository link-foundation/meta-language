#!/usr/bin/env node
// Partitions the JavaScript test files into the groups CI runs as separate
// jobs, so no single job runs the whole suite in one process.
//
//   node scripts/test-groups.mjs --check        every test file is in exactly one group
//   node scripts/test-groups.mjs --list GROUP   print the files of GROUP
//   node scripts/test-groups.mjs --run GROUP    run GROUP with node --test
//     [--shard K/N] (or TEST_GROUP_SHARD=K/N) partitions files exactly once
//
// A new test file that no group claims fails --check (and every --run), so a
// file can never silently drop out of CI.
import { spawnSync,execFileSync } from 'node:child_process';
import { readdirSync,readFileSync } from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {readProjectData,writeProjectData} from './project-data.mjs';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const schedulingFile=new URL('../../parity/fixtures/ci-39615628279-workload-costs.lino',import.meta.url);
const schedulingRevision='8bc0ef9725388c7927acf1854870bf46f5e365be';
const integrationSchedulingFile=new URL('../../parity/fixtures/ci-39644810104-workload-costs.lino',import.meta.url);
const integrationSchedulingRevision='8f2f6c483ec32a3ffc17afec694463965dc7551d';
const schedulingDigest=value=>createHash('sha256').update(value).digest('hex');

/** Largest observed workload first; source changes use a byte-size estimate.
 * Timing samples only choose owners. They never establish test completion. */
export function partitionMeasuredWorkloads(files,count,{readSource,costs}={}){
 assert.ok(Number.isSafeInteger(count)&&count>0&&count<=files.length);
 assert.equal(new Set(files).size,files.length,'workload source identities are unique');
 const measurements=costs?[costs]:[readProjectData(integrationSchedulingFile),readProjectData(schedulingFile)];
 const known=new Map();
 for(const measured of measurements){
 assert.equal(measured.schemaVersion,1);assert.equal(measured.model,'observed CI workload scheduling costs 1');
 assert.ok(measured.run===39615628279&&measured.revision===schedulingRevision
   ||measured.run===39644810104&&measured.revision===integrationSchedulingRevision,'a scheduling sample has its actual source commit');
 assert.ok(Array.isArray(measured.entries)&&measured.entries.length<=2048);
 const seen=new Set();
 for(const row of measured.entries){
  assert.deepEqual(Object.keys(row).sort(),['bytes','file','milliseconds','sha256']);
  assert.ok(typeof row.file==='string'&&!seen.has(row.file));seen.add(row.file);assert.match(row.sha256,/^[a-f0-9]{64}$/u);
  assert.ok(Number.isSafeInteger(row.bytes)&&row.bytes>0&&Number.isSafeInteger(row.milliseconds)&&row.milliseconds>0);
  if(!known.has(row.file))known.set(row.file,[]);known.get(row.file).push(row);
 }
 }
 const rates=measurements[0].entries.map(row=>row.milliseconds/row.bytes).sort((a,b)=>a-b);
 assert.ok(rates.length>0);const rate=rates[Math.floor(rates.length/2)];
 const read=readSource??(file=>readFileSync(path.join(packageRoot,'..',file)));
 const workloads=files.map(file=>{const bytes=Buffer.from(read(file)),sha256=schedulingDigest(bytes);
  const row=known.get(file)?.find(candidate=>candidate.bytes===bytes.length&&candidate.sha256===sha256);
  return {file,milliseconds:row?row.milliseconds:Math.max(1,Math.ceil(bytes.length*rate))};});
 workloads.sort((a,b)=>b.milliseconds-a.milliseconds||a.file.localeCompare(b.file));
 const loads=Array(count).fill(0),shards=Array.from({length:count},()=>[]);
 for(const row of workloads){const owner=loads.indexOf(Math.min(...loads));loads[owner]+=row.milliseconds;shards[owner].push(row.file);}
 assert.deepEqual(shards.flat().sort(),[...files].sort(),'every entire workload has exactly one owner');
 return shards.map(shard=>shard.sort());
}

/** Reproduce advisory cost samples from complete retained GitHub job logs. */
export function captureMeasuredWorkloadCosts(directory){
 const jobs=JSON.parse(readFileSync(path.join(directory,'jobs-final.json'),'utf8')).jobs;
 const run=JSON.parse(readFileSync(path.join(directory,'run-current.json'),'utf8'));
 assert.ok(run.id===39615628279&&run.head_sha===schedulingRevision
   ||run.id===39644810104&&run.head_sha===integrationSchedulingRevision);
 const original=file=>execFileSync('git',['--no-replace-objects','show',run.head_sha+':'+file],{cwd:path.join(packageRoot,'..'),maxBuffer:2*1024*1024,timeout:15000});
 const groups=partitionTests(testFiles()).groups;
 const titles=new Map();for(const name of [...groups.grammar,...(run.id===39644810104?groups.translation:[])]){
  const file='js/tests/'+name;let source;try{source=original(file).toString('utf8');}catch{continue;}
  for(const match of source.matchAll(/\btest\(\s*(['"])([^\n]*?)\1\s*,/gu)){
   if(titles.has(match[2])&&titles.get(match[2])!==file)titles.set(match[2],null);else titles.set(match[2],file);
  }
 }
 const costs=new Map(),logs=[];
 for(const job of jobs.filter(row=>/^Self-Translation Report \(\d+\/8\)$/u.test(row.name)||/Tests \(grammar, /u.test(row.name)
   ||run.id===39644810104&&/Tests \(translation, /u.test(row.name))){
  assert.equal(job.status,'completed','costs come from completed job logs, including partial progress before a recorded deadline');
  const address=path.join(directory,'job-'+job.id+'-small.txt'),bytes=readFileSync(address);
  const receipt=JSON.parse(readFileSync(path.join(directory,'job-'+job.id+'-small-receipt.json'),'utf8'));
  assert.equal(schedulingDigest(bytes),receipt.sha256);assert.equal(bytes.length,receipt.selectedBytes);
  const range=/^bytes 0-(\d+)\/(\d+)$/u.exec(receipt.contentRange);
  assert.ok(range&&Number(range[1])+1===Number(range[2])||receipt.status===200&&receipt.fetchedBytes===receipt.selectedBytes&&bytes.length<262144,'cost capture requires the complete bounded log');
  logs.push({job:job.id,name:job.name,bytes:bytes.length,sha256:receipt.sha256});
  let previous=null,title=null;
  for(const line of bytes.toString('utf8').split('\n')){
   const timestamp=Date.parse(line.slice(0,28));
   if(job.name.startsWith('Self-Translation Report (')){
    if(previous===null&&line.includes('self-translation report: resolving js/'))previous=timestamp;
    const match=/self-translation report: recorded (js\/src\/[^ ]+\.js) \(/u.exec(line);
    if(match){assert.ok(Number.isFinite(timestamp)&&previous!==null);assert.ok(!costs.has(match[1]));costs.set(match[1],Math.max(1,Math.round(timestamp-previous)));previous=timestamp;}
   }else{
    const text=line.replace(/^[0-9T:.Z-]+ /u,'');
    const match=/^# Subtest: (.*)$/u.exec(text);if(match)title=match[1];
    const duration=/^\s*duration_ms: ([0-9.]+)$/u.exec(text);
    if(duration&&title){const file=titles.get(title);if(file)costs.set(file,(costs.get(file)??0)+Math.max(1,Math.round(Number(duration[1]))));title=null;}
   }
  }
 }
 const entries=[...costs].sort(([a],[b])=>a.localeCompare(b)).map(([file,milliseconds])=>{const bytes=original(file);return{file,bytes:bytes.length,sha256:schedulingDigest(bytes),milliseconds};});
 assert.equal(logs.length,run.id===39644810104?20:14);assert.ok(entries.filter(row=>row.file.startsWith('js/src/')).length>300);
 assert.ok(entries.find(row=>row.file==='js/tests/issue-195-interchange-api-cli.test.js').milliseconds>=262000);
 const packet={schemaVersion:1,model:'observed CI workload scheduling costs 1',run:run.id,revision:run.head_sha,logs,entries,
  boundary:'Advisory elapsed-time samples from complete bounded CI logs. Recorded-module intervals include overlapped import resolution; test samples include only uniquely named literal test groups. Missing or changed source bytes use a size estimate. Every current complete module and test remains mandatory exactly once under unchanged job counts, commands and deadlines. Timing samples are not requirement observations or forecasts of passing CI.'};
 writeProjectData(run.id===39615628279?schedulingFile:integrationSchedulingFile,packet);return packet;
}

/** Test groups, each a pattern over the test file name without `.test.js`. */
export const TEST_GROUPS = Object.freeze({
  grammar:
    /^(?!issue-195-grammar-(?:profile-|native-|declared-settling$))(?!error-recovery-|parse-scaling$|parser-memory$|issue-195-parse-memory-budget$|lino-grammar-scaling$|unicode-input-)(?:grammar-|native-tree-projection-memory$|native-concept-reuse-normalization$|lino-|links-notation$|pdf-grammar$|default-cst-|tree-sitter-node-kind$|issue-195-merge-real-reconciliation$|language-|natural-language$|regions$|issue-195-grammar-|issue-195-interchange-)/u,
  'grammar-native':
    /^(?:issue-195-grammar-native-(?!recovery$)|issue-195-grammar-declared-settling$|native-development-oracles$|native-text-grammars$)/u,
  'grammar-recovery': /^(?:error-recovery-|issue-195-grammar-native-recovery$)/u,
  'grammar-scaling':
    /^(?:parse-scaling$|parser-memory$|issue-195-parse-memory-budget$|lino-grammar-scaling$|unicode-input-)/u,
  'grammar-csharp': /^issue-195-grammar-profile-csharp$/u,
  'grammar-elixir': /^issue-195-grammar-profile-elixir$/u,
  'grammar-ruby': /^issue-195-grammar-profile-ruby$/u,
  translation:
    /^(?!self-translation-report-artifacts$)(?:translation|typed-program-|typescript-erasure$|issue-195-translation-|issue-195-binding-rename$|issue-195-structured-transformations$|issue-195-faithful-behavior$|issue-195-semantics-proof-preservation$|issue-195-project-semantics$|self-translation(?:-|$))/u,
  'report-artifacts': /^self-translation-report-artifacts$/u,
  conformance:
    /^(?:four-language-conformance$|issue-195-conformance$|issue-195-generative$|parity$|issue-195-runtime-parity-evidence$|core$|structured-data-(?:links|records)$|decorators$|access$|query-|graphql-adapter$|concept-distinctions$|concept-records-lazy$|concept-records-seed-index$|global-concept-(?:space|archive-parser)$|discovered-concept-links$|concept-views$|link-network-binary$|network-projection$|link-id-boundaries$|resource-description-graphs$|resource-description-ntriples$|formal-concept-definition$|formal-function-links$|formal-function-declaration-values$|boolean-declaration-value-hosts$|shared-numeric-function-links$|foundation-models$|issue-195-concept-records$|issue-195-naming$|issue-195-naming-candidates$)/u,
  downstream:
    /^(?:downstream-|formal-ai-semantic-projections$|issue-195-downstream-|issue-195-rml-|issue-195-formal-ai-|issue-195-delivery$|issue-195-dependency-current-stable-delivery$|dependency-delivery$|crate-package-include$|package-release$|prepare-npm-auth$)/u,
  tooling:
    /^(?:cache-|project-data(?:-|$)|external-grammar-|assertion-version-ranges$|issue-195-rust-suite-partitions$|issue-195-(?:observation-links|record-storage|report-storage)$|issue-195-self-translation-report$|issue-195-associative-stack-sources$|issue-195-meta-theory-sources$|issue-195-acceptance|issue-195-evidence|issue-195-sources$|issue-195-documentation$|issue-195-vision$|issue-195-merge-enforcement$|issue-195-merge-quality-evidence$|dependency-(?:inventory|upgrades|downloaded-grammar-assets)$|test-groups$|ci-workflow-structure$|ci-cancellation-investigation$|ci-rocq-toolchain-cache$|export-parity$|own-codebase-baseline-naming-history$)/u,
});

export function testFiles(root = packageRoot) {
  return readdirSync(path.join(root, 'tests'))
    .filter((name) => name.endsWith('.test.js'))
    .sort();
}

/** Returns `{ groups, unassigned, ambiguous }` for the given test files. */
export function partitionTests(files) {
  const groups = Object.fromEntries(Object.keys(TEST_GROUPS).map((group) => [group, []]));
  const unassigned = [];
  const ambiguous = [];
  for (const file of files) {
    const stem = file.replace(/\.test\.js$/u, '');
    const matches = Object.entries(TEST_GROUPS).filter(([, pattern]) => pattern.test(stem)).map(([group]) => group);
    if (matches.length === 0) unassigned.push(file);
    else if (matches.length > 1) ambiguous.push(`${file} (${matches.join(', ')})`);
    else groups[matches[0]].push(file);
  }
  return { groups, unassigned, ambiguous };
}

export function partitionProblems({ groups, unassigned, ambiguous }) {
  return [
    ...unassigned.map((file) => `tests/${file} is in no test group of scripts/test-groups.mjs`),
    ...ambiguous.map((file) => `tests/${file} is in more than one test group`),
    ...Object.entries(groups).filter(([, files]) => files.length === 0).map(([group]) => `test group ${group} has no test files`),
  ];
}

export function partitionGroupFiles(files, shard = '1/1') {
  const match = /^([1-9][0-9]*)\/([1-9][0-9]*)$/u.exec(shard);
  const index = Number(match?.[1]), count = Number(match?.[2]);
  if (!Number.isSafeInteger(index) || !Number.isSafeInteger(count) || index > count || count > files.length) {
    throw new Error('test group shard needs K/N with 1 <= K <= N <= the number of files');
  }
  // Only the measured general-grammar and six-cell translation cohorts change owners. Native and
  // historical modulo cohorts retain their independent exact partition rules.
  if(count===6&&(files.includes('issue-195-interchange-api-cli.test.js')||files.includes('translation-scheduling-source-history.test.js')))return partitionMeasuredWorkloads(files.map(file=>'js/tests/'+file),count)[index-1].map(file=>file.slice(9));
  return files.filter((_, offset) => offset % count === index - 1);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if(process.argv[2]==='--capture-costs'){
    assert.equal(process.argv.length,4);const result=captureMeasuredWorkloadCosts(process.argv[3]);
    console.log('Captured '+result.entries.length+' advisory source-bound costs from '+result.logs.length+' complete CI job logs.');
    process.exit(0);
  }
  const [mode, group, ...options] = process.argv.slice(2);
  if (options.length && (options.length !== 2 || options[0] !== '--shard')) throw new Error('only --shard K/N is accepted after the group');
  const partition = partitionTests(testFiles());
  const problems = partitionProblems(partition);
  if (problems.length > 0) {
    console.error(`JavaScript test groups:\n- ${problems.join('\n- ')}`);
    process.exit(1);
  }
  if (mode === '--check') {
    const counts = Object.entries(partition.groups).map(([name, files]) => `${name} ${files.length}`);
    console.log(`JavaScript test groups cover every test file: ${counts.join(', ')}`);
  } else if ((mode === '--list' || mode === '--run') && group in partition.groups) {
    const files = partitionGroupFiles(partition.groups[group], options[1] ?? process.env.TEST_GROUP_SHARD ?? '1/1')
      .map((file) => path.join('tests', file));
    if (mode === '--list') {
      console.log(files.join('\n'));
    } else {
      const { status, signal } = spawnSync(process.execPath, ['--test', '--test-concurrency=1', ...files], {
        cwd: packageRoot,
        stdio: 'inherit',
      });
      process.exit(status ?? (signal ? 1 : 0));
    }
  } else {
    console.error(`usage: test-groups.mjs --check | --list GROUP | --run GROUP (groups: ${Object.keys(TEST_GROUPS).join(', ')})`);
    process.exit(2);
  }
}
