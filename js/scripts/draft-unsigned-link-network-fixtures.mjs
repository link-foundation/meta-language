// Declare inputs/oracles only. Never import or execute the profile under draft.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[
 {name:'empty graph',source:'',identities:[],next:'1'},
 {name:'adjacent IDs across the safe Number boundary',source:'(9007199254740991: 9007199254740992)\n(9007199254740992: 9007199254740993)\n(9007199254740993: 9007199254740991)',identities:['9007199254740991','9007199254740992','9007199254740993'],next:'9007199254740994'},
 {name:'maximum identity is an actual self link',source:'(18446744073709551615: 18446744073709551615)',identities:['18446744073709551615'],next:null},
 {name:'maximum reference does not exhaust a smaller allocator',source:'(1: 18446744073709551615 1 18446744073709551615)',identities:['1'],next:'2'},
 {name:'penultimate ID leaves exactly one allocation',source:'(18446744073709551614: 18446744073709551615)',identities:['18446744073709551614'],next:'18446744073709551615'},
 {name:'ordered recursive graph retains repeated edges',source:'(4294967295: 4294967296 18446744073709551615 4294967296)\n(4294967296: 4294967295)\n(18446744073709551615: 4294967295 18446744073709551615)',identities:['4294967295','4294967296','18446744073709551615'],next:null},
 {name:'metadata and registered point retain exact native spelling',source:'(9007199254740993: 9007199254740993 (meta: (t: concept) (n: 1) (term: meaning) (def: recursive%20meaning%00%F0%9F%98%80) (lang: en) (span: 1 23 2 3 4 5) (flags: 15) (reg: 1)))',identities:['9007199254740993'],next:'9007199254740994',registered:'meaning'},
];
writeProjectData(new URL('../../parity/fixtures/unsigned-link-network.lino',import.meta.url),{
 schemaVersion:1,model:'complete unsigned link identities and binary state 1',cases,
 valid:['1','4294967295','4294967296','9007199254740991','9007199254740992','9007199254740993','18446744073709551614','18446744073709551615'],
 invalid:['0','-1','1.0','1e3','+1',' 1','1 ','18446744073709551616','99999999999999999999'],
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024',linksNotation:'>=0.22.0 <0.23.0',binary:'MLU64N01 wrapping MLDSNP02'},
 assertions:['everyOriginalDecimalBitRetained','neighboringIdsNeverMerge','orderedRepeatedAndCyclicEdgesRetained','fullMetadataRetained','textNativeJavaScriptParity','exactAllocatorAndExhaustion','deletedAllocationHistory','cacheStateRetained','sourceFreeBinaryReconstruction','independentDataViewFraming','nativeJavaScriptActualBytesEqual','allTruncationsReject','noncanonicalAndCounterfeitDirectoriesReject','unchangedLegacyRefusals','fullEditableRuleGraphBothCodecsFourEmissions'],
 boundary:'Unexecuted future controls. The explicit exact-ID profile coexists with the preserved Number profile. Dense views are reversible coordinates, not concept equivalence. Full direct integration of every algorithm, schema and CLI with exact IDs remains required; new dense nodes need explicit original ID allocation.',
});
