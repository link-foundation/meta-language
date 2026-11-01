// Literal inputs only. No represented operation executes on import.
export const javascriptInspectableCollectionVersions={node:'>=22.15 <23 || >=24 <25',profile:'inspectable global collection slots 1'};
export const javascriptInspectableCollectionDecisions=[
 {operation:'testJavaScriptCollectionKey',arguments:[{$:'number',value:-0},{$:'number',value:0}],expected:true},
 {operation:'testJavaScriptCollectionKey',arguments:[{$:'number',value:NaN},{$:'number',value:NaN}],expected:true},
 {operation:'testJavaScriptCollectionKey',arguments:[{$:'number',value:0},{$:'integer',value:0n}],expected:false},
 {operation:'testJavaScriptCollectionKey',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\ud800'},{$:'text',units:[55296]}],expected:true},
 {operation:'testJavaScriptCollectionKey',arguments:[{$:'object',identity:9n,callable:false},{$:'object',identity:10n,callable:false}],expected:false},
 {operation:'testJavaScriptCollectionKey',arguments:[{$:'symbol',identity:9n},{$:'object',identity:9n,callable:false}],expected:false},
 {operation:'normalizeJavaScriptCollectionKey',arguments:[{$:'number',value:-0}],expected:{$:'number',value:0}},
 {operation:'normalizeJavaScriptCollectionKey',arguments:[{$:'integer',value:0n}],expected:{$:'integer',value:0n}},
 {operation:'readJavaScriptCollectionOperationKind',arguments:['Map.get'],expected:'Map'},
 {operation:'readJavaScriptCollectionOperationKind',arguments:['Set.add'],expected:'Set'},
 {operation:'readJavaScriptCollectionOperationKind',arguments:['Set.set'],expected:''},
 {operation:'compareJavaScriptCollectionKeys',arguments:[{$:'number',value:NaN},{$:'number',value:Infinity}],expected:1},
 {operation:'compareJavaScriptCollectionKeys',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'a'},{$:'text',units:[97]}],expected:0},
 {operation:'orderJavaScriptCollectionKeys',arguments:[[{$:'text',units:[98]},{$:'number',value:NaN},{$:'number',value:-0},{$:'text',units:[97]}]],expected:[{$:'number',value:-0},{$:'number',value:NaN},{$:'text',units:[97]},{$:'text',units:[98]}]}
];
