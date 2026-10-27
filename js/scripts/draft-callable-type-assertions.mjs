// Source/data authoring: parse only the independent assertion type grammar.
// Root assertions check function items. Nested fn types use the existing Rust
// callable value encoding; source owner trees retain original primitive atoms.
export function draftCallableTypeAssertion(source){
 const prefix='const _: ';if(!source.startsWith(prefix))throw new TypeError('a complete independent function assertion is required');let at=prefix.length;
 function eat(value){if(source.slice(at,at+value.length)===value){at+=value.length;return true;}return false;}
 function need(value){if(!eat(value))throw new TypeError('incomplete independent type assertion at '+at);}
 function type(){if(eat('fn')){need('(');const params=[];while(!eat(')')){params.push(type());if(eat(')'))break;need(',');}return{kind:'function',params,result:eat('->')?type():{kind:'unit'}};}
  if(eat('(')){if(eat(')'))return{kind:'unit'};const elements=[type()];if(!eat(',')){need(')');return elements[0];}while(!eat(')')){elements.push(type());if(eat(')'))break;need(',');}return{kind:'tuple',elements};}
  const match=source.slice(at).match(/^[a-z][a-z0-9]*/u);if(!match)throw new TypeError('an actual independent primitive type is required');at+=match[0].length;if(!['bool','f64','i8','u8','i16','u16','i32','u32','i64','u64','i128','u128','isize','usize'].includes(match[0]))throw new TypeError('an independent supported primitive is required');return{kind:'primitive',name:match[0]};
 }
 function canonical(value,root=false){if(value.kind==='unit')return'()';if(value.kind==='primitive')return value.name==='usize'?'u64':value.name==='isize'?'i64':value.name;if(value.kind==='tuple')return '('+value.elements.map(item=>canonical(item)).join(',')+(value.elements.length===1?',':'')+')';const parameters=value.params.map(item=>canonical(item)).join(',');return root?'fn('+parameters+')->'+canonical(value.result):'std::rc::Rc<dyn Fn('+parameters+')->'+canonical(value.result)+'>';}
 function owner(value){return value.kind==='primitive'?{kind:'primitive',path:[value.name],arguments:[]}:value.kind==='unit'?{kind:'unit',path:[],arguments:[]}:value.kind==='tuple'?{kind:'tuple',path:[],arguments:value.elements.map(owner)}:{kind:'function',path:[],arguments:[...value.params.map(owner),owner(value.result)]};}
 const parsed=type();if(parsed.kind!=='function')throw new TypeError('the independent root must describe an actual function item');need('=CASE::');const name=source.slice(at).match(/^[A-Za-z_][A-Za-z_0-9]*/u)?.[0];if(!name)throw new TypeError('an actual independent source declaration is required');at+=name.length;need(';');if(at!==source.length)throw new TypeError('the whole independent source assertion is required');
 return{canonical:prefix+canonical(parsed,true)+'=CASE::'+name+';',sourceContract:{name,parameters:parsed.params.map(owner),result:owner(parsed.result)}};
}
