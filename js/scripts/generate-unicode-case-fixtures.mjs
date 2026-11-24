// Expected declarations only. Original Node and translated calls run later.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const inputs=[
 ['ASCII lower','return "AbC-Z".toLowerCase();','abc-z'],
 ['ASCII upper','return "AbC-z".toUpperCase();','ABC-Z'],
 ['full sharp s expansion','return "straße".toUpperCase();','STRASSE'],
 ['full dotted I expansion','return "İ".toLowerCase();','i\u0307'],
 ['ligature expansion','return "ﬃ".toUpperCase();','FFI'],
 ['final sigma','return "ΟΣ".toLowerCase();','ος'],
 ['initial sigma','return "Σ".toLowerCase();','σ'],
 ['two sigma original context','return "ΣΣ".toLowerCase();','σς'],
 ['nonfinal sigma','return "ΑΣΑ".toLowerCase();','ασα'],
 ['final sigma ignores trailing combining mark','return "ΑΣ\\u0301".toLowerCase();','ας\u0301'],
 ['nonfinal sigma ignores combining mark before a cased letter','return "ΑΣ\\u0301Β".toLowerCase();','ασ\u0301β'],
 ['overlapping cased ignorable preceding sigma is possessive','return "\\u0345Σ".toLowerCase();','\u0345σ'],
 ['preceding cased letter before overlapping ignorable','return "Α\\u0345Σ".toLowerCase();','α\u0345ς'],
 ['following overlapping ignorable does not make sigma nonfinal','return "ΑΣ\\u0345".toLowerCase();','ας\u0345'],
 ['following cased letter after overlapping ignorable','return "ΑΣ\\u0345Α".toLowerCase();','ασ\u0345α'],
 ['default casing excludes Turkish locale tailoring','return "Iİıi".toLowerCase();','ii\u0307ıi'],
 ['supplementary Deseret lower','return "\\u{10400}".toLowerCase();','\u{10428}'],
 ['supplementary Deseret upper','return "\\u{10428}".toUpperCase();','\u{10400}'],
 ['Armenian expansion','return "և".toUpperCase();','ԵՒ'],
 ['Georgian Mtavruli lower','return "\\u1c90".toLowerCase();','ა'],
 ['empty case map','return "".toUpperCase();',''],
 ['isolated units remain exact around mapped letters','const value="\\ud800ΟΣ\\udc00".toLowerCase();return value.length===4&&value.charCodeAt(0)===55296&&value.charCodeAt(3)===56320&&value.slice(1,3)==="ος";','true'],
 ['extra argument effects precede case mapping','let calls=0;const value="ß".toUpperCase(calls=3);return value+calls;','SS3'],
 ['receiver evaluation occurs once','let calls=0;const value=(calls+=1,"İ").toLowerCase();return value+calls;','i\u03071'],
 ['own record method keeps its callable meaning','const value={toLowerCase:()=>7};return value.toLowerCase();','7'],
];
const cases=inputs.map(([name,body,value],index)=>({name,source:'export function compute(){'+body+'}',entry:'unicodeCaseValue'+index,value,abort:null}));
writeProjectData(new URL('../../parity/fixtures/unicode-case-values.lino',import.meta.url),{
 schemaVersion:1,model:'Unicode 16 original source default casing values 1',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'Node.js >=24.0.0 <25.0.0 with Unicode 16.0; ECMAScript 2025 default casing',Rust:'>=1.90.0 <1.100.0; edition 2024; generated Unicode 16 tables, independent of host character mapping'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/text-processing.html#sec-string.prototype.tolowercase ; https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/#G33992 ; https://www.unicode.org/Public/16.0.0/ucd/SpecialCasing.txt',
 boundary:'Future original Node/emitted JavaScript/native values, complete public source definitions, both codecs and four targets. No observations. Original-oracle execution must select actual Unicode 16.0; unknown profiles refuse. Locale-specific casing, alternate Unicode profiles, generic receivers and every preceding requirement remain.',
});
