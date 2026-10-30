// Pinned upstream implementation references, not execution observations.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/translation/number-negation-profile.lino',import.meta.url),{
 schemaVersion:1,model:'scalar Number unary negation bit profile 1',nodeVersionRanges:['>=22.15.0 <23.0.0','>=24.0.0 <25.0.0'],rustVersionRange:'>=1.90.0 <1.100.0',
 operation:'Negate the original Number scalar through unary negation. Do not substitute multiplication or first collect the scalar in a Number-only array.',
 sources:[
  {repository:'nodejs/node',ref:'v22.15.0',path:'deps/v8/src/ic/unary-op-assembler.cc',gitBlobSha:'dd9d5af06bd95c8d06a95d208a3446ef480e8d94',url:'https://github.com/nodejs/node/blob/v22.15.0/deps/v8/src/ic/unary-op-assembler.cc'},
  {repository:'nodejs/node',ref:'v24.0.0',path:'deps/v8/src/ic/unary-op-assembler.cc',gitBlobSha:'61d3a72575d8499c09e31fbcdb666c2800cdac12',url:'https://github.com/nodejs/node/blob/v24.0.0/deps/v8/src/ic/unary-op-assembler.cc'},
  {repository:'nodejs/node',ref:'v22.15.0',path:'deps/v8/src/objects/fixed-array-inl.h',gitBlobSha:'344ab262f0725a28b4a0d4136aa3f845bb664436',url:'https://github.com/nodejs/node/blob/v22.15.0/deps/v8/src/objects/fixed-array-inl.h'},
  {repository:'nodejs/node',ref:'v24.0.0',path:'deps/v8/src/objects/fixed-array-inl.h',gitBlobSha:'696deaf3187471730c9b86393b1ace8e74d8b320',url:'https://github.com/nodejs/node/blob/v24.0.0/deps/v8/src/objects/fixed-array-inl.h'},
  {repository:'nodejs/node',ref:'v24.0.0',path:'deps/v8/src/codegen/code-stub-assembler.cc',gitBlobSha:'9071cb487d72942a15fc86edd92d00203c19bbab',url:'https://github.com/nodejs/node/blob/v24.0.0/deps/v8/src/codegen/code-stub-assembler.cc'}
 ],
 boundary:'Source inspection motivates the bounded assertions. No local engine probe, fixture execution, runtime observation, universal NaN encoding claim or completion evidence is recorded.'
});
