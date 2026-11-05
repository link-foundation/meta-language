// Literal control data; neither source programs nor kernels run on import.
export const javascriptArrayObjectVersions={node:'>=22.15 <23 || >=24 <25',profile:'inspectable sparse Array property definitions 1'};
export const javascriptArrayObjectDecisions=[
 {operation:'testJavaScriptArrayLength',arguments:[0],expected:true},
 {operation:'testJavaScriptArrayLength',arguments:[4294967295],expected:true},
 {operation:'testJavaScriptArrayLength',arguments:[4294967296],expected:false},
 {operation:'testJavaScriptArrayLength',arguments:[-0],expected:false},
 {operation:'testJavaScriptArrayLength',arguments:[NaN],expected:false},
 {operation:'readJavaScriptArrayIndex',arguments:[{$:'text',units:[48]}],expected:0},
 {operation:'readJavaScriptArrayIndex',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'4294967294'}],expected:4294967294},
 {operation:'readJavaScriptArrayIndex',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'4294967295'}],expected:-1},
 {operation:'readJavaScriptArrayIndex',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'01'}],expected:-1},
 {operation:'convertJavaScriptArrayUnsignedLength',arguments:[-1],expected:4294967295},
 {operation:'convertJavaScriptArrayUnsignedLength',arguments:[-0],expected:0},
 {operation:'convertJavaScriptArrayUnsignedLength',arguments:[Infinity],expected:0},
 {operation:'convertJavaScriptArrayUnsignedLength',arguments:[4294967297],expected:1},
 {operation:'testJavaScriptArrayLengthKey',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'length'}],expected:true},
 {operation:'testJavaScriptArrayLengthKey',arguments:[{$:'text',units:[48]}],expected:false},
];
