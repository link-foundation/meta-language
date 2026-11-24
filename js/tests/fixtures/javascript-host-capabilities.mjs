// Independent literal decisions, not authority receipts or observations.
export const javascriptHostCapabilityDecisions=[
 {operation:'testJavaScriptHostCapabilityMode',arguments:['qualified host capability'],expected:true},
 {operation:'testJavaScriptHostCapabilityMode',arguments:['Error intrinsic'],expected:false},
 {operation:'testJavaScriptHostCapabilityMode',arguments:['strict'],expected:false},
 {operation:'testJavaScriptHostCapabilityMode',arguments:['lexical'],expected:false},
 {operation:'testJavaScriptHostCapabilityMode',arguments:['Qualified host capability'],expected:false},
 {operation:'testJavaScriptHostCapabilityMode',arguments:[''],expected:false},
];
