// Callable bounds need complete source positions as well as erased core types.
// In particular usize/u64 and reference lifetimes must not collapse here.
export function testRustFnMutSourcePosition(parser,left,right){
 if(!left||!right||!parser.origins.same(left,right)||!Array.isArray(left.wrappers)||!Array.isArray(right.wrappers)||left.wrappers.length!==right.wrappers.length||left.wrappers.some((value,index)=>value!==right.wrappers[index]))return false;
 const first=left.sourceLifetimes??[],second=right.sourceLifetimes??[];if(first.length!==second.length||first.some((value,index)=>value!==second[index]))return false;
 const pending=[[left.expandedSourceOwner??left.sourceOwner,right.expandedSourceOwner??right.sourceOwner]];let count=0;
 while(pending.length){if(++count>65536)return false;const [a,b]=pending.pop();if(!a||!b||a.kind!==b.kind||!Array.isArray(a.path)||!Array.isArray(b.path)||!Array.isArray(a.arguments)||!Array.isArray(b.arguments)||a.path.length!==b.path.length||a.path.some((value,index)=>value!==b.path[index])||a.arguments.length!==b.arguments.length)return false;for(let at=0;at<a.arguments.length;at++)pending.push([a.arguments[at],b.arguments[at]]);}return true;
}
