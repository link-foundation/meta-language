// Actual source expressions into existing shared formatting and abort effects.
import {UNIT} from './types.js';
import {acceptRustPanicFormattingValue,readRustPanicFormattingPrefix,readRustPanicValueOperation} from './rust-panic-value-rules.js';
export function readRustFormattedPanic(parser,token,args,names,range,path){
 const first=args[0],text=first?.k==='str'?first.value:'';
 if(!acceptRustPanicFormattingValue(token.value,parser.ordinaryCompletionContext(),first?.k??'',args.length,text.includes('{')||text.includes('}')))return null;
 const formatted=parser.format(args,token,path,names),prefix=readRustPanicFormattingPrefix(token.value),value=prefix?{k:'binary',op:'concat',left:{k:'str',value:prefix,span:range},right:formatted,span:range}:formatted;
 const result={k:'math',op:readRustPanicValueOperation(),name:'Rust formatted panic message',args:[{value,spread:false}],span:range};
 return parser.origins.mark(result,{...parser.origins.typeOrigin(UNIT,path),diverging:true});
}
