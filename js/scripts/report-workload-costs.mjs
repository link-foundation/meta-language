// Advisory scheduling data only; every report module and assertion still runs.
import assert from 'node:assert/strict';
import {readProjectData} from './project-data.mjs';
import {partitionMeasuredWorkloads} from './test-groups.mjs';
const file=new URL('../../parity/fixtures/ci-39857274367-report-costs.lino',import.meta.url);
export function partitionReportMeasuredWorkloads(files,count,options={}){
 const costs=options.costs??readProjectData(file);
 assert.ok(costs.run===39848242758&&costs.revision==='d3e4076667add3e19e4cf5f10a3cea60255dd2d7'
  ||costs.run===39857274367&&costs.revision==='f3853187c9e3395409361b60b4abe1608cdf1998');
 assert.equal(costs.logs.length,8);assert.equal(costs.inventory.length,costs.entries.length);
 assert.deepEqual(costs.entries.map(row=>row.file),costs.inventory);
 return partitionMeasuredWorkloads(files,count,{...options,costs});
}

// Split a complete logical report cell without changing its module inventory.
export function selectReportPhysicalPart(files,part,options={}){
 assert.equal(typeof part,'string');
 const match=/^([1-9][0-9]*)\/([1-9][0-9]*)$/u.exec(part);
 assert.ok(match,'--physical-part takes K/N');
 const index=Number(match[1]),count=Number(match[2]);
 assert.ok(Number.isSafeInteger(index)&&Number.isSafeInteger(count)&&index<=count&&count<=files.length,'invalid physical report geometry');
 return partitionReportMeasuredWorkloads(files,count,options)[index-1];
}
