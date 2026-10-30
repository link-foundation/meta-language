import {readRustPrimitiveConstantPattern} from './rust-constant-patterns.js';
// Actual tokens and checked literal values into the shared interval compiler.
import {readRustIntegerRangeSourceError,readRustIntegerRangeSuccessor,readRustIntegerRangeBoundsError,readRustIntegerRangeCuts,planRustIntegerRangeRows} from './rust-integer-range-rules.js';
import {fixedBounds} from './types.js';
import {unsupported,typeError} from './diagnostics.js';
export function readRustIntegerRangePattern(parser,path,first,lower=null){
 const c=parser.cursor;if(!c.is('..')&&!c.is('..='))return lower;
 const operation=c.next();let upper=null;
 if(c.is('-')&&c.isKind('number',1)){const sign=c.next();upper=parser.numericPattern(c.next(),sign,true);}
 else if(c.isKind('number')){const token=c.next();upper=parser.numericPattern(token,token,false);}
 else if(c.isKind('byteChar')){const token=c.next();upper={k:'byteLit',value:token.value,span:{start:token.start,end:token.end}};}
 else upper=readRustPrimitiveConstantPattern(parser,path);
 const where={start:first.start,end:upper?.span?.end??operation.end};
 const error=readRustIntegerRangeSourceError(parser.ordinaryCompletionContext(),operation.value,Boolean(lower),Boolean(upper));if(error)throw unsupported('range pattern',error,where);
 return {k:'integerRange',lower,upper,inclusive:operation.value==='..=',span:where};
}
export function normaliseRustIntegerRangePattern(checker,pattern,type,span){
 if(type.kind!=='fixed')throw typeError('an integer range pattern requires an actual fixed-width integer scrutinee',pattern.span??span);
 const bounds=fixedBounds(type),end=endpoint=>{
  const normal=checker.normalisePattern(endpoint,type,span);
  if(normal.k!=='lit')throw typeError('an integer range endpoint requires an actual checked literal',endpoint.span??span);
  return normal.key.slice(normal.key.lastIndexOf(':')+1);
 };
 const lower=pattern.lower?end(pattern.lower):String(bounds.min),upper=pattern.upper?(pattern.inclusive?readRustIntegerRangeSuccessor(end(pattern.upper)):end(pattern.upper)):readRustIntegerRangeSuccessor(String(bounds.max));
 const error=readRustIntegerRangeBoundsError(lower,upper);if(error)throw typeError(error,pattern.span??span);
 return{k:'interval',lower,upper};
}
export function compileRustIntegerRangeRows(checker,columns,rows,pivot,span){
 const column=columns[pivot],bounds=fixedBounds(column.type),kinds=[],lowers=[],uppers=[];
 for(const row of rows){const pattern=row.patterns[pivot];kinds.push(pattern.k);if(pattern.k==='interval'){lowers.push(pattern.lower);uppers.push(pattern.upper);}else if(pattern.k==='lit'){const value=pattern.key.slice(pattern.key.lastIndexOf(':')+1);lowers.push(value);uppers.push(readRustIntegerRangeSuccessor(value));}else{lowers.push('');uppers.push('');}}
 const cuts=readRustIntegerRangeCuts(String(bounds.min),readRustIntegerRangeSuccessor(String(bounds.max)),lowers,uppers),plan=planRustIntegerRangeRows(kinds,lowers,uppers,cuts);
 if(plan[0]!==0)throw typeError(plan[0]===-2?'non-exhaustive match':'integer range compilation requires its complete checked domain partition',span);
 const remaining=columns.filter((_,index)=>index!==pivot),bodies=[];let at=2;
 for(let cell=0;cell<plan[1];cell+=1){const count=plan[at++],selected=[];for(let index=0;index<count;index+=1){const row=rows[plan[at++]],pattern=row.patterns[pivot];selected.push({...row,patterns:row.patterns.filter((_,index)=>index!==pivot),binds:pattern.k==='bind'?[...row.binds,[pattern.name,column.name]]:row.binds});}bodies.push(checker.compileRows(remaining,selected,span));}
 let body=bodies.at(-1);
 for(let cell=bodies.length-2;cell>=0;cell-=1){const text=cuts[cell+1],negative=text.startsWith('-');body={k:'if',cond:{k:'binary',op:'lt',left:{k:'name',path:[column.name]},right:{k:'num',value:negative?text.slice(1):text,type:column.type,negative}},then:bodies[cell],else:body,span};}
 return body;
}

export function readRustIntegerPatternOwners(pattern){
 if(!pattern)return [];
 if(pattern.k==='alias')return readRustIntegerPatternOwners(pattern.pattern);
 if(pattern.k==='integerRange')return [...readRustIntegerPatternOwners(pattern.lower),...readRustIntegerPatternOwners(pattern.upper)];
 return [pattern.k==='byteLit'?'u8':pattern.k==='typedNumLit'?pattern.owner:''];
}
