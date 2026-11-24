// Source declarations only. Actual endpoint and policy controls run later in CI.
import{writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[
 {words:[1,0,0,0],exhausted:false,count:1,expected:true},
 {words:[1,0,0,0],exhausted:false,count:4294967295,expected:true},
 {words:[0,0,65534,65535],exhausted:false,count:4294967295,expected:true},
 {words:[0,0,65535,65535],exhausted:false,count:4294967295,expected:true},
 {words:[65535,65535,65535,65535],exhausted:false,count:1,expected:true},
 {words:[65535,65535,65535,65535],exhausted:false,count:2,expected:false},
 {words:[65534,65535,65535,65535],exhausted:false,count:2,expected:true},
 {words:[65534,65535,65535,65535],exhausted:false,count:3,expected:false},
 {words:[0,0,0,0],exhausted:true,count:0,expected:true},
 {words:[0,0,0,0],exhausted:true,count:1,expected:false},
 {words:[0,0,0,0],exhausted:false,count:0,expected:false},
 {words:[1,0,0,0],exhausted:true,count:0,expected:false},
 {words:[65536,0,0,0],exhausted:false,count:1,expected:false},
 {words:[1,0.5,0,0],exhausted:false,count:1,expected:false},
 {words:[1,0,0,0],exhausted:false,count:-1,expected:false},
 {words:[1,0,0,0],exhausted:false,count:4294967296,expected:false},
];
const endpoints=[18446744073709551615n,18446744073709551614n,18446744073709551613n].map(value=>({maximum:String(value),source:'('+value+': '+value+' (meta: (t: concept) (n: 1) (term: maximum) (reg: 1)))\n'}));
writeProjectData(new URL('../../parity/fixtures/link-allocation.lino',import.meta.url),{
 schemaVersion:1,model:'actual unsigned 64-bit link allocation capacity 1',cases,endpoints,
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 boundary:'Future exact policy decisions in both runtimes against independent BigInt capacity arithmetic, full definition graph/codecs/four emissions, actual native maximum-u64 text imports, final allocation, self-link object insertion and atomic field/storage exhaustion. No observations. Full JavaScript u64 identifiers and a matching exact binary/archive profile remain separate obligations.',
});
