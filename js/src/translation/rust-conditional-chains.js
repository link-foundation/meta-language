// Actual source scopes into the shared continuation plan. Packets ensure that
// failure never evaluates or captures an else expression under a matched binder.
import {unsupported} from './diagnostics.js';
import {findRustConditionalChainPattern,planConditionalChainLayers,readConditionalChainOperation,readConditionalChainStageLimit,planConditionalMatchArms} from './conditional-chain-rules.js';
const where=(from,to)=>({start:from.start,end:Math.max(from.end,to&&to.kind!=='eof'?to.start:from.end)});
const operation=(role,values,span)=>({k:'math',op:readConditionalChainOperation(role),name:'',args:values.map(value=>({value,spread:false})),span});
export function hasRustConditionalChain(parser){const tokens=parser.cursor.tokens;return findRustConditionalChainPattern(tokens.map(token=>token.kind),tokens.map(token=>token.value),parser.cursor.index)>=0;}
export function readRustConditionalChainStages(parser,path,start){
 const c=parser.cursor,stages=[],forms=[];
  do{
   const token=c.peek();
   if(c.eat('let')){
    const aliases=[],patterns=[parser.pattern(path,aliases)];while(c.eat('|'))patterns.push(parser.pattern(path,aliases));
    if(patterns.length>1&&aliases.length)throw unsupported('binding in an or-pattern','bind the value in each alternative separately',where(start,c.peek()));
    c.expect('=','conditional pattern');const scrutinee=parser.comparison(path,{noStruct:true});
    parser.origins.enter(parser.origins.cases(patterns,scrutinee,path));for(const alias of aliases)parser.origins.bind(alias.name,alias.value,path);
    stages.push({patterns,scrutinee,aliases,span:where(token,c.peek())});forms.push(1);
   }else{
    const condition=parser.comparison(path,{noStruct:true});parser.origins.direct(condition,path,condition.span);
    stages.push({condition,span:where(token,c.peek())});forms.push(0);
   }
   if(c.is('||'))throw unsupported('Rust conditional pattern disjunction','a let condition requires parentheses around each Boolean disjunction',where(start,c.peek()));
   if(stages.length>readConditionalChainStageLimit())throw unsupported('Rust conditional pattern chain depth','the actual condition exceeds its shared stage budget',where(start,c.peek()));
  }while(c.eat('&&'));
 return {stages,forms};
}
export function lowerRustConditionalChainStages({stages,forms},body,otherwise,span){
 let result=operation('success',[body],span);const failed=operation('failure',[],span),plan=planConditionalChainLayers(forms);
 if(plan.length!==stages.length*2)throw unsupported('Rust conditional chain transport','every actual stage needs its complete shared layer',span);
 for(let at=0;at<plan.length;at+=2){const stage=stages[plan[at+1]];if(plan[at]===0)result={k:'if',cond:stage.condition,then:result,else:failed,span:stage.span};else{
   for(const alias of [...stage.aliases].reverse())result={k:'let',name:alias.name,value:alias.value,body:result,span:stage.span};
   result={k:'match',scrutinees:[stage.scrutinee],rows:[...stage.patterns.map(pattern=>({patterns:[pattern],body:result})),{patterns:[{k:'wild'}],body:failed}],span:stage.span};
  }}
 return {...operation('result',[result,otherwise],span),block:true};
}
export function readRustConditionalChain(parser,path,start){
 const c=parser.cursor,previous=parser.origins.enter();let syntax,body,bodyOrigin;
 try{syntax=readRustConditionalChainStages(parser,path,start);body=parser.block(path);bodyOrigin=parser.origins.origin(body,path);}
 finally{parser.origins.scope=previous;}
 if(!c.eat('else'))throw unsupported('if without else','the value of an if must be defined on both branches',where(start,c.peek()));
 const otherwise=c.is('if')?parser.ifExpr(path):parser.block(path),span=where(start,c.peek()),origin=parser.origins.join([bodyOrigin,parser.origins.origin(otherwise,path)]);
 return parser.origins.mark(lowerRustConditionalChainStages(syntax,body,otherwise,span),origin);
}

function aliasesAround(aliases,body,span){for(const alias of [...aliases].reverse())body={k:'let',name:alias.name,value:alias.value,body,span};return body;}
export function lowerRustGuardedMatch(parser,scrutinee,arms,path,span){
 // Each actual alternative is a separate attempted match. A false guard on an
 // earlier alternative must allow the next alternative to try its guard again.
 arms=arms.flatMap(arm=>arm.patterns.map(pattern=>({...arm,patterns:[pattern]})));
 const plan=planConditionalMatchArms(arms.map(arm=>arm.guard?1:0));
 if(!plan.length)throw unsupported('Rust guarded match depth','the actual arms exceed their shared stage budget',span);
 const name=parser.freshProductBinding(),bound={k:'name',path:[name],span},failed=operation('failure',[],span);
 parser.origins.mark(bound,parser.origins.origin(scrutinee,path));
 const rows=arm=>arm.patterns.map(pattern=>({patterns:[pattern],body:aliasesAround(arm.aliases,arm.body,arm.span),span:arm.span}));
 const coverage={k:'match',scrutinees:[bound],rows:plan.slice(1,1+plan[0]).flatMap(at=>rows(arms[at])),span};
 let result=failed;
 for(const at of plan.slice(1+plan[0])){
  const arm=arms[at];let body=operation('success',[operation('success',[arm.body],arm.span)],arm.span);
  if(arm.guard)body={k:'if',cond:arm.guard,then:body,else:failed,span:arm.span};
  body=aliasesAround(arm.aliases,body,arm.span);
  const matched={k:'match',scrutinees:[bound],rows:[...arm.patterns.map(pattern=>({patterns:[pattern],body,span:arm.span})),{patterns:[{k:'wild'}],body:failed}],span:arm.span};
  result=operation('result',[matched,result],span);
 }
 const value={k:'let',name,value:scrutinee,body:operation('result',[result,coverage],span),span,block:true};
 return parser.origins.mark(value,parser.origins.join(arms.map(arm=>arm.origin)));
}
