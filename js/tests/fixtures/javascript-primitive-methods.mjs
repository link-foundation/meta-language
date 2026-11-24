export const javascriptPrimitiveMethodVersions={node:'>=22.15 <23 || >=24 <25',profile:'actual standard primitive method identities 1'};
export const javascriptPrimitiveMethodDecisions=[
 {operation:'executeJavaScriptPrimitiveMethod',arguments:[{$:'text',units:[97,98,99]},'String.startsWith',[{$:'text',units:[98]},{$:'number',value:1}]],expected:{$:'primitiveValue',value:{$:'boolean',value:true}}},
 {operation:'executeJavaScriptPrimitiveMethod',arguments:[{$:'sourceText',profile:'inspectable UTF16 source text 1',text:'abc'},'String.startsWith',[{$:'text',units:[98]},{$:'number',value:2}]],expected:{$:'primitiveValue',value:{$:'boolean',value:false}}},
 {operation:'executeJavaScriptPrimitiveMethod',arguments:[{$:'text',units:[97,98,99]},'String.startsWith',[{$:'text',units:[97]},{$:'number',value:-Infinity},{$:'number',value:19}]],expected:{$:'primitiveValue',value:{$:'boolean',value:true}}},
 {operation:'executeJavaScriptPrimitiveMethod',arguments:[{$:'text',units:[97,98,99]},'String.startsWith',[{$:'text',units:[]},{$:'number',value:Infinity}]],expected:{$:'primitiveValue',value:{$:'boolean',value:true}}},
 {operation:'executeJavaScriptPrimitiveMethod',arguments:[{$:'text',units:[97,98,99]},'String.startsWith',[{$:'text',units:[97]},{$:'number',value:NaN}]],expected:{$:'primitiveValue',value:{$:'boolean',value:true}}},
 {operation:'executeJavaScriptPrimitiveMethod',arguments:[{$:'text',units:[97,98,99]},'String.startsWith',[{$:'text',units:[97]},{$:'number',value:-0}]],expected:{$:'primitiveValue',value:{$:'boolean',value:true}}},
 {operation:'executeJavaScriptPrimitiveMethod',arguments:[{$:'undefined'},'String.startsWith',[]],expected:{$:'primitiveError',errorClass:'TypeError',reason:'String.prototype.startsWith called on null or undefined'}},
 {operation:'executeJavaScriptPrimitiveMethod',arguments:[{$:'text',units:[97]},'String.startsWith',[{$:'object',identity:1n,callable:false}]],expected:{$:'primitiveInvalid',reason:'String search requires its actual IsRegExp and ToPrimitive protocols'}}
];
export const javascriptPrimitiveAdditionDecisions=[
 {operation:'executeJavaScriptCallPrimitive',arguments:['add primitive values',[{$:'text',units:[97]},{$:'number',value:2}]],expected:{$:'primitiveValue',value:{$:'text',units:[97,50]}}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['add primitive values',[{$:'boolean',value:true},{$:'null'}]],expected:{$:'primitiveValue',value:{$:'number',value:1}}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['add primitive values',[{$:'number',value:-0},{$:'number',value:-0}]],expected:{$:'primitiveValue',value:{$:'number',value:-0}}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['add primitive values',[{$:'integer',value:1n},{$:'integer',value:2n}]],expected:{$:'primitiveValue',value:{$:'integer',value:3n}}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['add primitive values',[{$:'integer',value:1n},{$:'number',value:2}]],expected:{$:'primitiveError',errorClass:'TypeError',reason:'Cannot mix BigInt and other types, use explicit conversions'}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['add primitive values',[{$:'integer',value:1n},{$:'symbol',identity:2n}]],expected:{$:'primitiveError',errorClass:'TypeError',reason:'Cannot convert a Symbol value to a number'}},
 {operation:'executeJavaScriptCallPrimitive',arguments:['add primitive values',[{$:'object',identity:1n,callable:false},{$:'text',units:[97]}]],expected:{$:'primitiveInvalid',reason:'addition requires its actual ordered object ToPrimitive continuations'}}
];
