// Actual lexical declaration transport through the shared object-pattern plan.
import {unsupported} from './diagnostics.js';
import {requireJavaScriptIdentifierRole} from './javascript-identifier-roles.js';
import {readJavaScriptObjectBindingStatement} from './javascript-record-parameter-rules.js';
import {readJavaScriptRecursiveObjectBindingPositions} from './recursive-binding-rules.js';
import {readJavaScriptRecursiveObjectPlan,requireJavaScriptRecursiveObjectPlan,constructJavaScriptRecursiveObjectEntries} from './recursive-object-bindings.js';
export function readJavaScriptObjectBindingNames(parser,start){
 const {tokens,plan}=readJavaScriptRecursiveObjectPlan(parser,start);
 return readJavaScriptRecursiveObjectBindingPositions(plan).map(position=>tokens[position]);
}
export function readJavaScriptObjectBindingDeclaration(parser,startToken,mode){
 const c=parser.cursor,start=c.index,syntax=readJavaScriptRecursiveObjectPlan(parser,start),{tokens,plan}=syntax;
 requireJavaScriptRecursiveObjectPlan(parser,syntax,startToken);
 if(tokens[0]?.value==='['&&mode==='let'&&parser.topLevel)throw unsupported('let destructuring','mutable literal bindings require a function-owned lexical scope',{start:startToken.start,end:tokens[0].end});
 if(readJavaScriptObjectBindingStatement(mode)==='')throw unsupported('object binding transport','the declaration needs its complete actual object pattern and lexical mode',c.peek());
 const positions=readJavaScriptRecursiveObjectBindingPositions(plan),bindings=positions.map(position=>tokens[position]);
 for(const token of bindings){requireJavaScriptIdentifierRole(token,true);parser.lexicalDeclaration(token.value,{start:token.start,end:token.end});parser.scope.tdz.add(token.value);}
 c.index=start+plan[0];c.expect('=','destructuring');const value=parser.expr(),where={start:startToken.start,end:c.peek().end};
 const name=parser.generatedPrefix+'temporary_binding'+(++parser.generatedCount),object={k:'name',path:[name],span:where};
 const array=tokens[0]?.value==='[';
 const entries=constructJavaScriptRecursiveObjectEntries(parser,syntax,object,{positions,context:array?'ordinary recursive array binding declaration':'ordinary object binding declaration',profile:array?'array binding initializer':'object binding initializer',fresh:()=>parser.generatedPrefix+'object_binding_value'+(++parser.generatedCount)});
 const statements=[{s:'const',name,value,span:where},...entries.map(entry=>({s:entry.temporary?'const':readJavaScriptObjectBindingStatement(mode),name:entry.name,value:entry.value,span:where}))];
 for(const token of bindings){parser.declareLocal(token.value,mode==='let');parser.lexicalBinding(token.value,c.peek().start);}
 return {statements,bindings,span:where};
}
