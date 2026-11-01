// Independent literal requirements. Complete provider bytes are retained by
// each future producer capture; these rows never compute policy expectations.
export const projectDataSourceVersions={node:'>=22.15 <23 || >=24 <25',linksNotation:'0.22.0'};
export const projectDataSourceOriginals=[
 {name:'explicit JSON reads the actual source branch',files:[{name:'value.json',text:'{"answer":42,"nested":{"value":"first"}}'}],file:'value.json',options:{format:'json'},entry:'readProjectData',expected:{answer:42,nested:{value:'first'}},expectedError:null},
 {name:'auto JSON path prefers the actual LiNo sibling',files:[{name:'value.lino',data:{answer:17}}],file:'value.json',options:{format:'auto'},entry:'readProjectData',expected:{answer:17},expectedError:null},
 {name:'explicit LiNo path reaches the shared decoder capability',files:[{name:'value.lino',data:{items:['first','second'],active:true}}],file:'value.lino',options:{format:'lino'},entry:'readProjectData',expected:{items:['first','second'],active:true},expectedError:null},
 {name:'actual URL path flows through pathString',files:[{name:'value.json',text:'{"url":true}'}],file:'value.json',asURL:true,entry:'readProjectData',expected:{url:true},expectedError:null},
 {name:'destructured option defaults use actual empty argument',files:[{name:'value.json',text:'{"defaulted":true}'}],file:'value.json',entry:'readProjectData',expected:{defaulted:true},expectedError:null},
 {name:'actual duplicate-storage branch throws before decode',files:[{name:'value.json',text:'{}'},{name:'value.lino',data:{}}],file:'value.json',entry:'readProjectData',expected:null,expectedError:{name:'Error',messagePrefix:'project data has duplicate JSON and LiNo storage: '}},
 {name:'format rejection uses the actual selectedFormat helper',files:[],file:'value.json',options:{format:'yaml'},entry:'readProjectData',expected:null,expectedError:{name:'TypeError',message:'project data format must be auto, json or lino'}},
 {name:'actual unsupported extension follows raw-read source order',files:[{name:'value.txt',text:'unstructured'}],file:'value.txt',entry:'readProjectData',expected:null,expectedError:{name:'TypeError',messagePrefix:'project data has an unsupported storage extension: '}},
 {name:'resolve explicit JSON bypasses sibling probes',files:[],file:'missing.json',options:{format:'json'},entry:'resolveProjectDataPath',expectedPath:'missing.json',expectedError:null},
 {name:'resolve nonstorage suffix preserves actual path',files:[],file:'value.txt',entry:'resolveProjectDataPath',expectedPath:'value.txt',expectedError:null}
];
export const projectDataSourceRequiredControls=[
 {name:'actual large snapshot successor',requirement:'at least 8 MiB UTF-16 text and at most 48 MiB UTF-8; actual current text read every call; cache clone fresh with nested alias preservation; external edit invalidates once; complete literal payload and assertions retained by bounded producer'},
 {name:'same imported reader instance',requirement:'initialize two import aliases with successor reader profile, share one module cache and actual source-provider instance; caller const aliases share same loaded semantic handle'},
 {name:'actual reader profile predecessor',requirement:'old qualified runtime Links data IO adapter 1 remains explicit and its complete source/assertions remain unchanged'},
 {name:'decoder body boundary',requirement:'retain complete structured-data-links.js bytes/hash and package version as external qualification; shared decoder implementation translation remains required and is not inferred from source capture'},
 {name:'shared source semantics transport',requirement:'actual source plan, registry, bindings, capabilities and compiled program survive exact Links text/binary codec transport, then execute same VM branch/IO ordering; generated native source runs same shared compiler'}
];
export const projectDataSourceCacheOriginals=[
 {name:'actual large cache clones and invalidates current bytes',prefixUnit:' ',prefixCount:8388608,first:{nested:{value:'first'},active:true},afterEdit:{nested:{value:'second'},active:false},expectedReads:3,expectedDecodes:2,expectedValues:['first','first','second'],expectedDistinctRootHandles:true,expectedDistinctNestedHandles:true}
];
export const projectDataSourceRefusals=[
 {name:'unbound source effects',source:'export function readProjectData(file){ return unknown(file); }',entries:['readProjectData'],reason:'actual unshadowed bounded operation owner'},
 {name:'unmodeled top-level initialization',source:'const value=load(); export function readProjectData(file){ return file; }',entries:['readProjectData'],reason:'source initialization path'},
 {name:'shadowed native constructor has no imported capability',source:'export function readProjectData(URL){ return new URL("file:///value.json"); }',entries:['readProjectData'],reason:'unshadowed source capability'},
 {name:'unknown selected export',source:'function readProjectData(file){ return file; }',entries:['readProjectData'],reason:'actual exported source function'},
 {name:'custom loop statements need actual lowering',source:'export function readProjectData(file){ while(file){ file=next(file); } return file; }',entries:['readProjectData'],reason:'actual unshadowed bounded operation owner'}
];
