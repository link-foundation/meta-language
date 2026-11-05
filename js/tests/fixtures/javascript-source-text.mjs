// Literal successor controls. Existing unit-array text controls stay intact.
export const javascriptSourceTextVersions={node:'>=22.15 <23 || >=24 <25',profile:'inspectable UTF16 source text 1',maximumCodeUnits:16777216};
export const javascriptSourceTextProfileDecisions=[
 {operation:'readJavaScriptSourceTextProfileError',arguments:['inspectable UTF16 source text 1',0],expected:''},
 {operation:'readJavaScriptSourceTextProfileError',arguments:['inspectable UTF16 source text 1',8388608],expected:''},
 {operation:'readJavaScriptSourceTextProfileError',arguments:['inspectable UTF16 source text 1',16777216],expected:''},
 {operation:'readJavaScriptSourceTextProfileError',arguments:['inspectable UTF16 source text 1',16777217],expected:'source text exceeds the explicit 16777216 UTF16 unit budget'},
 {operation:'readJavaScriptSourceTextProfileError',arguments:['opaque host string',8],expected:'source text requires its explicit inspectable UTF16 resource profile'},
 {operation:'readJavaScriptSourceTextProfileError',arguments:['inspectable UTF16 source text 1',-0],expected:'source text exceeds the explicit 16777216 UTF16 unit budget'},
 {operation:'readJavaScriptSourceTextProfileError',arguments:['inspectable UTF16 source text 1',NaN],expected:'source text exceeds the explicit 16777216 UTF16 unit budget'}
];
export const javascriptSourceTextSemanticDecisions=[
 {operation:'readJavaScriptSemanticValueError',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\ud800x\udfff'}],expected:''},
 {operation:'testJavaScriptSemanticSameValue',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\ud800x\udfff'},{$:'text',units:[55296,120,57343]}],expected:true},
 {operation:'testJavaScriptSemanticSameValue',arguments:[{$:'text',units:[55296,120,57343]},{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\ud800x\udfff'}],expected:true},
 {operation:'testJavaScriptSemanticSameValue',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'a'},{$:'text',units:[98]}],expected:false},
 {operation:'testJavaScriptSemanticSameValue',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:''},{$:'text',units:[]}],expected:true},
 {operation:'readJavaScriptSemanticTextLength',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'💩'}],expected:2},
 {operation:'readJavaScriptSemanticTextUnit',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'💩'},1],expected:56489},
 {operation:'readJavaScriptSemanticTextUnit',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'💩'},2],expected:-1},
 {operation:'readJavaScriptSemanticValueError',arguments:[{$:'sourceText',profile:'unqualified',text:'a'}],expected:'source text requires its explicit inspectable UTF16 resource profile'}
];
export const javascriptSourceTextBooleanDecisions=[
 {operation:'convertJavaScriptSemanticBoolean',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:''}],expected:false},
 {operation:'convertJavaScriptSemanticBoolean',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'0'}],expected:true},
 {operation:'convertJavaScriptSemanticBoolean',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\u0000'}],expected:true},
 {operation:'convertJavaScriptSemanticBoolean',arguments:[{$:'number',value:-0}],expected:false},
 {operation:'convertJavaScriptSemanticBoolean',arguments:[{$:'number',value:NaN}],expected:false}
];
export const javascriptSourceTextPrimitiveDecisions=[
 {operation:'executeJavaScriptCallPrimitive',arguments:['read semantic type',[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'hello'}]],expected:{$:'primitiveValue',value:{$:'text',units:[115,116,114,105,110,103]}}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['read text length',[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\ud800x'}]],expected:{$:'primitiveValue',value:{$:'number',value:2}}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['convert primitive text',[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\ud800x'}]],expected:{$:'primitiveValue',value:{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\ud800x'}}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['concatenate text',[{$:'text',units:[55296]},{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'x'}]],expected:{$:'primitiveValue',value:{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'\ud800x'}}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['convert explicit number',[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'-0'}]],expected:{$:'primitiveValue',value:{$:'number',value:-0}}}
];
export const javascriptSourceTextLargeControls=[
 {name:'old unit-array resource bound remains 1048576 code units',unit:120,count:1048577,expectedOldError:'semantic text exceeds the explicit UTF16 unit budget',expectedSuccessorError:''},
 {name:'compact full text larger than old bound',text:'x',count:8388608,expectedLength:8388608,expectedType:'string',expectedConversion:'primitiveValue',expectedUnsupportedSlice:'primitiveResourceLimit'},
 {name:'compact text maximum resource bound',text:'x',count:16777216,expectedError:'',overflow:16777217,expectedOverflowError:'source text exceeds the explicit 16777216 UTF16 unit budget'}
];
