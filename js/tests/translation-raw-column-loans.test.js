import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readPortableSourceParameterStyles} from '../scripts/portable-source-library-style.mjs';
import {readRustLibraryItems} from '../scripts/rust-library-packaging.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {readOriginalRawColumnLoanStyles} from '../../experiments/original-link-complete-table-projection/raw-column-loan-styles.mjs';
import {ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT,readOriginalLinkRawConsumingProfile} from '../../experiments/original-link-complete-table-projection/raw-consuming-emission.mjs';
import {prepareOriginalLinkRawBorrowedEmission} from '../../experiments/original-link-complete-table-projection/raw-borrowed-emission.mjs';

let control;
function columnLoanControl() {
  if(!control){
    const stored=readProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
    const source=readPortableSourceParameterStyles(stored.completeRust,{ownedParameters:{check_original_link_raw_shape:['columns']}}).source;
    control={stored,source,lowered:readOriginalRawColumnLoanStyles(source)};
  }
  return control;
}

test('read-only column loans retain every complete checked owner and restore the whole original source',()=>{
  const {source,lowered}=columnLoanControl();assert.ok(lowered.record.loans.length>=26);
  const before=readRustLibraryItems(source).items,after=readRustLibraryItems(lowered.source).items;
  assert.deepEqual(after.map(({kind,name})=>({kind,name})),before.map(({kind,name})=>({kind,name})));
  assert.equal(before.filter(item=>item.kind==='fn').length,64);
  assert.equal(before.filter(item=>item.kind==='fn'&&/^pub fn /mu.test(item.source)).length,62);
  const headers=items=>items.filter(item=>item.kind==='fn').map(item=>item.source.slice(0,item.source.indexOf('{')));
  assert.deepEqual(headers(after),headers(before),'every actual public and continuation interface is unchanged');
  let restored=lowered.source;for(const action of [...lowered.actions].reverse()){assert.equal(restored.split(action.to).length,2);restored=restored.replace(action.to,()=>action.from);}
  assert.equal(restored,source);
  assert.equal(DecoratorSet.fromLino(lowered.record.decorators).decorate('emitter',{format:'Rust',source}).source,lowered.source);
  assert.ok(lowered.record.loans.every(row=>row.readOnlyUses>0&&row.index>=0&&row.index<=65535));
});

test('the actual borrowed receipt replays every local loan without selecting a native process or changing its predecessor',()=>{
  const {stored,lowered}=columnLoanControl(),predecessor=readOriginalLinkRawConsumingProfile(stored.predecessor);
  const actual=prepareOriginalLinkRawBorrowedEmission(predecessor);
  assert.equal(actual.source,lowered.source);assert.deepEqual(actual.receipt.columnLoans,lowered.record);
  assert.deepEqual(actual.profile.proof.predecessor,predecessor);
  assert.equal(actual.receipt.nativeCompilation,false);assert.equal(actual.receipt.nativeExecution,false);
});

test('immutable primitive column selection retains every original bit and the missing-column empty default',()=>{
  const {lowered}=columnLoanControl();
  const bits=[0n,0x8000000000000000n,0x3ff0000000000000n,0xbff0000000000000n,
    0x7ff0000000000000n,0xfff0000000000000n,0x7ff8000000000123n,1n,0x000fffffffffffffn,0x7fefffffffffffffn];
  for(let length=0;length<=24;length++){
    const columns=Array.from({length},(_,index)=>Array.from({length:index%7},(_,at)=>bits[(index+at)%bits.length]));
    const original=columns.map(row=>row.slice());
    for(const loan of lowered.record.loans){
      const copied=columns[loan.index]?.slice().slice()??[],borrowed=columns[loan.index]??[];
      assert.deepEqual(borrowed,copied);
      assert.equal(borrowed.length,copied.length);
    }
    assert.deepEqual(columns,original,'every caller-owned primitive bit stays unchanged');
  }
});

test('unreviewed source effects captures writes and input expansions refuse before any lowering',()=>{
  const {source}=columnLoanControl();
  for(const changed of [source+' ',source.replace('let kinds =','let mut kinds ='),
    source.replace('crate::ml_array::optional_at(&kinds,','observe_pointer(&kinds,')]){
    assert.notEqual(changed,source);assert.throws(()=>readOriginalRawColumnLoanStyles(changed),/complete reviewed borrowed raw source/u);
  }
});
