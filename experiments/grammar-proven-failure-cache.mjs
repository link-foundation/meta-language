// Experimental instrumentation, separate from the production executor.
// Reuse a completed empty answer only when a conservative FIRST calculation
// excludes the current input independently of all recursive seeds. The original
// evaluation has already recorded its actual diagnostics and memory charge.
// The recorded JavaScript/TypeScript trial admitted zero entries and still hit
// its step limits. This remains a diagnostic experiment, with no production
// activation or established workload improvement.
import {Executor} from '../js/src/grammar-runtime/executor.js';
import {encodeText} from '../js/src/grammar-runtime/text.js';

export function readGrammarFirstPrefixes(grammar) {
  const rules=new Map([...grammar.rules.keys()].map(name=>[name,{nullable:false,unknown:false,first:new Map()}]));
  const keys=new WeakMap();
  const empty=()=>({nullable:true,unknown:false,first:new Map()});
  const unknown=()=>({...empty(),unknown:true});
  const key=node=>{if(!keys.has(node))keys.set(node,JSON.stringify(node));return keys.get(node);};
  const union=(values,nullable)=>{
    const first=new Map();let unresolved=false;
    for(const value of values){unresolved||=value.unknown;for(const [name,item]of value.first)first.set(name,item);}
    return {nullable,unknown:unresolved||first.size>64,first:first.size>64?new Map():first};
  };
  const read=node=>{
    if(node.kind==='empty')return empty();
    // Lookahead can consult scanners or other contextual operations even when
    // it consumes nothing. This experiment supplies no certificate for it.
    if(['and','not'].includes(node.kind))return unknown();
    if(node.kind==='literal'){
      // Only the ordinary, already-decoded literal has the exact UTF-8
      // semantics used below. Case-folded or unfamiliar literal settings refuse.
      if(typeof node.value!=='string'||Reflect.ownKeys(node).some(name=>name!=='kind'&&name!=='value'))return unknown();
      if(node.value==='')return empty();
      const first={kind:'literal',value:[...node.value][0]};
      return {nullable:false,unknown:false,first:new Map([[JSON.stringify(first),first]])};
    }
    if(['charClass','charRange','byteClass','any'].includes(node.kind))
      return {nullable:false,unknown:false,first:new Map([[key(node),node]])};
    if(node.kind==='ref')return node.arguments?.length?unknown():rules.get(node.name)??unknown();
    if(node.kind==='choice'){
      const values=node.items.map(read);return union(values,values.some(value=>value.nullable));
    }
    if(node.kind==='seq'){
      const values=[];let nullable=true;
      for(const item of node.items){const value=read(item);values.push(value);if(!value.nullable){nullable=false;break;}}
      return union(values,nullable);
    }
    if(['optional','repeat0','repeat1','repeat'].includes(node.kind)){
      const value=read(node.item);return {...value,nullable:node.kind==='optional'||node.kind==='repeat0'||node.min===0||value.nullable};
    }
    if(['capture','alias','precedence','namedPrecedence','dynamicPrecedence','lexicalPrecedence','token','atomic'].includes(node.kind))return read(node.item);
    // Immediate tokens, scanners, state actions, imports, parameters and other
    // context-sensitive forms deliberately provide no exclusion certificate.
    return unknown();
  };
  for(let round=0;round<80;round++){
    let changed=false;
    for(const [name,rule]of grammar.rules){
      const before=rules.get(name),after=rule.parameters?.length||rule.action?unknown():read(rule.expression);
      if(after.nullable!==before.nullable||after.unknown!==before.unknown||after.first.size!==before.first.size||[...after.first.keys()].some(value=>!before.first.has(value))){rules.set(name,after);changed=true;}
    }
    if(!changed)return {rules,trivia:union((grammar.declarations.extras??[]).map(read),true),rounds:round+1};
  }
  return null;
}

export function instrumentProvenGrammarFailures() {
  const previous=Executor.prototype.run,prefixCache=new WeakMap();
  const statistics={admitted:0,refused:0};
  Executor.prototype.run=function(...arguments_){
    if(!prefixCache.has(this.program))prefixCache.set(this.program,readGrammarFirstPrefixes({
      rules:this.program.rules,declarations:{extras:this.program.trivia.map(item=>item.expression)},
    }));
    const prefixes=prefixCache.get(this.program);
    if(!prefixes)return previous.apply(this,arguments_);
    const executor=this,memo=this.memo,remove=memo.delete;
    const names=new Map([...this.program.rules].map(([name,rule])=>[rule,name]));
    const matches=(first,position)=>[...first.values()].some(item=>{
      if(item.kind!=='literal')return executor.matchTerminal(item,position)>=0;
      // Synthesized FIRST literals are not keys in program.matchers. Compare
      // the same decoded UTF-8 bytes as load.js's ordinary literal matcher.
      const literal=encodeText(item.value);
      return position+literal.length<=executor.end&&literal.every((byte,index)=>executor.bytes[position+index]===byte);
    });
    memo.delete=function(key){
      const entry=this.get(key),prefix=entry?.rule?prefixes.rules.get(names.get(entry.rule)):null;
      if(entry&&!entry.evaluating&&entry.involved&&entry.results?.length===0
        &&executor.repairPoints===null&&executor.suppressed===0
        &&this.size<=executor.memoLimit&&prefix&&!prefix.nullable&&!prefix.unknown
        &&!prefixes.trivia.unknown&&!matches(prefixes.trivia.first,entry.position)
        &&!matches(prefix.first,entry.position)){
        this.set(key,{results:entry.results,keywordContext:entry.keywordContext});
        statistics.admitted++;return false;
      }
      statistics.refused++;return remove.call(this,key);
    };
    try{return previous.apply(this,arguments_);}finally{memo.delete=remove;}
  };
  return {statistics,restore(){Executor.prototype.run=previous;}};
}
