// Complete source composition decisions. Hosts supply actual lexer columns;
// no filesystem, parser callback or saved target text supplies these rules.
import {readRequiredTextFact,readRequiredNumberFact} from './translation/fact-column-rules.js';

/** @param {string[][]} columns @param {number} index @returns {string[]} */
function textColumn(columns,index){if(index<0||index>=columns.length)throw new RangeError('missing Rust project text column');return columns[index]??[];}
/** @param {number[][]} columns @param {number} index @returns {number[]} */
function numberColumn(columns,index){if(index<0||index>=columns.length)throw new RangeError('missing Rust project Number column');return columns[index]??[];}
/** @param {string[]} kinds @param {string[]} values @param {number} index @returns {number} */
function groupEnd(kinds,values,index){
 const open=readRequiredTextFact(values,index),stack=[open==='{'?'}':open==='['?']':')'];
 for(let position=index+1;position<values.length;position+=1){
  if(readRequiredTextFact(kinds,position)!=='punct')continue;
  const value=readRequiredTextFact(values,position);
  if(value==='{'||value==='['||value==='(')stack.push(value==='{'?'}':value==='['?']':')');
  else if(value==='}'||value===']'||value===')'){
   if(value!==readRequiredTextFact(stack,stack.length-1))throw new TypeError('unbalanced Rust project source group');
   stack.pop();if(stack.length===0)return position+1;
  }
 }
 throw new TypeError('unclosed Rust project source group');
}
/** Five Number columns per actual module: keyword, name, delimiter, after,
 * parent record. Nongrammatical owner bodies and macro inputs are skipped as
 * complete groups, so their text cannot invent a module declaration.
 * @param {string[]} kinds @param {string[]} values @returns {number[]} */
export function readRustProjectModulePlan(kinds,values){
 if(kinds.length!==values.length)throw new TypeError('unaligned Rust project token columns');
 const plan=[],parents=[],closes=[];let position=0,parent=-1;
 while(position<values.length){
  const kind=readRequiredTextFact(kinds,position),value=readRequiredTextFact(values,position);
  if(closes.length>0&&position===readRequiredNumberFact(closes,closes.length-1)){closes.pop();parent=readRequiredNumberFact(parents,parents.length-1);parents.pop();position+=1;continue;}
  if(kind==='keyword'&&value==='mod'){
   if(position+2>=values.length||readRequiredTextFact(kinds,position+1)!=='identifier')throw new TypeError('invalid Rust project module name');
   const delimiter=readRequiredTextFact(values,position+2);
   if(readRequiredTextFact(kinds,position+2)!=='punct'||delimiter!==';'&&delimiter!=='{')throw new TypeError('invalid Rust project module boundary');
   const after=delimiter===';'?position+3:groupEnd(kinds,values,position+2),record=plan.length/5;
   plan.push(position,position+1,position+2,after,parent);
   if(delimiter==='{'){parents.push(parent);closes.push(after-1);parent=record;position+=3;}else position=after;
   continue;
  }
  if(kind==='punct'&&(value==='{'||value==='['||value==='(')){
   if(value==='['&&position>0&&(readRequiredTextFact(values,position-1)==='#'||position>1&&readRequiredTextFact(values,position-1)==='!'&&readRequiredTextFact(values,position-2)==='#')){
    const attribute=position+1<values.length?readRequiredTextFact(values,position+1):'';
    if(attribute==='cfg'||attribute==='cfg_attr'||attribute==='macro_use')throw new TypeError('Rust project module attributes need an explicit configuration interpretation');
   }
   position=groupEnd(kinds,values,position);continue;
  }
  if(kind==='punct'&&(value==='}'||value===']'||value===')'))throw new TypeError('unexpected Rust project closing group');
  position+=1;
 }
 return plan;
}
/** @param {string} path @returns {boolean} */
export function acceptRustProjectSourcePath(path){
 if(!path||path.startsWith('/')||path.includes('\\')||path.includes('\0')||path.includes(':'))return false;
 for(const part of path.split('/'))if(!part||part==='.'||part==='..')return false;
 return true;
}
/** Three Number facts per module: literal token, attribute start and after.
 * Only actual outer attributes attached to an actual planned module count.
 * Function bodies and macro inputs cannot invent a path interpretation.
 * @param {string[]} kinds @param {string[]} values @param {number[]} plan
 * @returns {number[]} */
export function readRustProjectModulePathAttributes(kinds,values,plan){
 let selected=[];
 let position=0;
 while(position<values.length){
  const kind=readRequiredTextFact(kinds,position),value=readRequiredTextFact(values,position);
  if(kind==='punct'&&value==='#'&&position+1<values.length){
   const inner=readRequiredTextFact(values,position+1)==='!',open=position+(inner?2:1);
   if(open>=values.length||readRequiredTextFact(kinds,open)!=='punct'||readRequiredTextFact(values,open)!=='['){position+=1;continue;}
   const after=groupEnd(kinds,values,open);
   if(open+1<values.length&&readRequiredTextFact(values,open+1)==='path'){
    if(inner||after!==open+5||readRequiredTextFact(values,open+2)!=='='||readRequiredTextFact(kinds,open+3)!=='string')throw new TypeError('Rust project path attribute needs an outer literal module path');
    let owner=after;
    while(owner+1<values.length&&readRequiredTextFact(values,owner)==='#'&&readRequiredTextFact(values,owner+1)==='[')owner=groupEnd(kinds,values,owner+1);
    if(owner<values.length&&readRequiredTextFact(values,owner)==='pub'){
     owner+=1;if(owner<values.length&&readRequiredTextFact(values,owner)==='(')owner=groupEnd(kinds,values,owner);
    }
    if(owner<values.length&&readRequiredTextFact(values,owner)==='unsafe')owner+=1;
    let record=-1;for(let at=0;at<plan.length;at+=5)if(readRequiredNumberFact(plan,at)===owner)record=at/5;
    if(record<0)throw new TypeError('Rust project path attribute needs its actual module owner');
    if(selected.length>0&&readRequiredNumberFact(selected,selected.length-4)===record)throw new TypeError('duplicate Rust project module path attributes');
    selected=[...selected,record,open+3,position,after];
   }
   position=after;continue;
  }
  if(kind==='punct'&&(value==='{'||value==='['||value==='(')){
   let inline=false;
   if(value==='{')for(let at=0;at<plan.length;at+=5)if(readRequiredNumberFact(plan,at+2)===position)inline=true;
   position=inline?position+1:groupEnd(kinds,values,position);continue;
  }
  position+=1;
 }
 const attributes=[];let selectedAt=0;
 for(let record=0;record*5<plan.length;record+=1){
  if(selectedAt<selected.length&&readRequiredNumberFact(selected,selectedAt)===record){attributes.push(readRequiredNumberFact(selected,selectedAt+1),readRequiredNumberFact(selected,selectedAt+2),readRequiredNumberFact(selected,selectedAt+3));selectedAt+=4;}
  else attributes.push(-1,-1,-1);
 }
 return attributes;
}
/** Normalize only within the complete supplied inventory. Parent components
 * are meaningful Rust paths; escaping the project or selecting host-specific
 * absolute/drive/backslash paths has no interpretation in this profile.
 * @param {string} directory @param {string} path @returns {string} */
export function resolveRustProjectLiteralPath(directory,path){
 if(path.startsWith('/')||path.includes('\\')||path.includes('\0')||path.includes(':'))throw new TypeError('nonportable Rust project literal path');
 let result=directory?directory.split('/'):[];
 for(const part of path.split('/')){
  if(part===''||part==='.')continue;
  if(part==='..'){if(result.length===0)throw new TypeError('Rust project literal path escapes supplied inventory');result=result.slice(0,-1);}
  else result=[...result,part];
 }
 return result.join('/');
}
/** Return directory and outstanding ordinary-file stem. A path-loaded source
 * is root-like even when its physical filename is not mod.rs.
 * @param {string} directory @param {string} relative @param {number[]} plan
 * @param {number[]} attributes @param {string[]} values @param {number} record
 * @returns {string[]} */
function moduleSourceContext(directory,relative,plan,attributes,values,record){
 let parent=readRequiredNumberFact(plan,record*5+4),ancestors=[];
 while(parent>=0){ancestors=[parent,...ancestors];parent=readRequiredNumberFact(plan,parent*5+4);}
 let current=directory,pending=relative;
 for(const ancestor of ancestors){
  const literal=readRequiredNumberFact(attributes,ancestor*3);
  if(literal>=0)current=resolveRustProjectLiteralPath(current,readRequiredTextFact(values,literal));
  else current=[current,pending,readRequiredTextFact(values,readRequiredNumberFact(plan,ancestor*5+1))].filter(part=>part!=='').join('/');
  pending='';
 }
 return [current,pending];
}
/** Preserve every ordinary source interval; a resolved outer path attribute
 * becomes an explicit zero-output interpretation interval, never lost input.
 * @param {string} source @param {string} path
 * @param {number} from @param {number} to @param {string} owner
 * @param {number[]} plan @param {number[]} attributes @param {string[]} values
 * @param {number[]} starts @param {number[]} ends @returns {string[]} */
function readSourceIntervals(source,path,from,to,owner,plan,attributes,values,starts,ends){
 let offset=from;const events=[];
 // Modules follow source order, including nested inline declarations.
 for(let record=0;record*3<attributes.length;record+=1){
  const literal=readRequiredNumberFact(attributes,record*3);if(literal<0)continue;
  const first=readRequiredNumberFact(starts,readRequiredNumberFact(attributes,record*3+1)),last=readRequiredNumberFact(ends,readRequiredNumberFact(attributes,record*3+2)-1);
  if(first<from||first>=to)continue;
  if(first<offset||last>to)throw new TypeError('overlapping Rust project interpreted attribute interval');
  events.push('source',path,String(offset),String(first),source.slice(offset,first),owner,'');
  const ancestors=moduleAncestors(plan,values,record),name=readRequiredTextFact(values,readRequiredNumberFact(plan,record*5+1));
  events.push('interpreted attribute',path,String(first),String(last),'',owner+'::'+[...ancestors,name].join('::'),readRequiredTextFact(values,literal));
  offset=last;
 }
 events.push('source',path,String(offset),String(to),source.slice(offset,to),owner,'');return events;
}
/** Root files and mod.rs use their parent directory; an ordinary module file
 * owns a same-named directory for its submodules.
 * @param {string} path @param {boolean} root @returns {string} */
export function readRustProjectModuleDirectory(path,root){
 const parts=path.split('/'),file=readRequiredTextFact(parts,parts.length-1),parent=parts.slice(0,-1);
 return [...parent,...(!root&&file!=='mod.rs'?[file.slice(0,-3)]:[])].join('/');
}
/** @param {string} directory @param {string} name @param {string[]} available @returns {string} */
export function selectRustProjectModuleSource(directory,name,available){
 const prefix=directory?directory+'/':'',first=prefix+name+'.rs',second=prefix+name+'/mod.rs';
 const foundFirst=available.includes(first),foundSecond=available.includes(second);
 if(foundFirst&&foundSecond)throw new TypeError('ambiguous Rust project module files: '+first+' and '+second);
 if(!foundFirst&&!foundSecond)throw new TypeError('missing Rust project module file: '+first+' or '+second);
 return foundFirst?first:second;
}
/** Actual inline parents, in source order; the active external file supplies
 * its own owning directory separately.
 * @param {number[]} plan @param {string[]} values @param {number} record @returns {string[]} */
function moduleAncestors(plan,values,record){
 let parent=readRequiredNumberFact(plan,record*5+4),ordered=[];
 while(parent>=0){ordered=[readRequiredTextFact(values,readRequiredNumberFact(plan,parent*5+1)),...ordered];parent=readRequiredNumberFact(plan,parent*5+4);}
 return ordered;
}
/** Seven text fields per event: kind, original file, original UTF16 start/end,
 * exact emitted fragment, qualified module, selected provider. Every original
 * file remains a separate caller input. Events keep replaced semicolons and
 * synthetic braces explicit; they never claim equivalence between concepts.
 * @param {string[]} paths @param {string[]} sources
 * @param {string[][]} kinds @param {string[][]} values
 * @param {number[][]} starts @param {number[][]} ends
 * @param {string} entry @param {number} depthLimit
 * @param {number} expansionLimit @param {number} outputUnitLimit
 * @returns {string[]} */
export function expandRustProjectSourceEvents(paths,sources,kinds,values,starts,ends,entry,depthLimit,expansionLimit,outputUnitLimit){
 if(paths.length!==sources.length||paths.length!==kinds.length||paths.length!==values.length||paths.length!==starts.length||paths.length!==ends.length)throw new TypeError('unaligned Rust project source inventory');
 if(!Number.isSafeInteger(depthLimit)||depthLimit<1||!Number.isSafeInteger(expansionLimit)||expansionLimit<1||!Number.isSafeInteger(outputUnitLimit)||outputUnitLimit<1)throw new RangeError('invalid Rust project resource limit');
 const unique=new Set(paths);if(unique.size!==paths.length)throw new TypeError('duplicate Rust project source path');
 const plans=[],pathAttributes=[];
 for(let file=0;file<paths.length;file+=1){
  if(!acceptRustProjectSourcePath(readRequiredTextFact(paths,file)))throw new TypeError('invalid Rust project source path');
  const types=textColumn(kinds,file),tokens=textColumn(values,file),first=numberColumn(starts,file),last=numberColumn(ends,file);
  if(types.length!==first.length||types.length!==last.length)throw new TypeError('unaligned Rust project token ranges');
  const plan=readRustProjectModulePlan(types,tokens);plans.push(plan);pathAttributes.push(readRustProjectModulePathAttributes(types,tokens,plan));
 }
 const root=paths.indexOf(entry);if(root<0)throw new TypeError('missing Rust project entry source');
 // Each continuation holds file, next module record, retained offset, depth.
 let frames=[root,0,0,0],directories=[readRustProjectModuleDirectory(entry,true)],relativeDirectories=[''],owners=['crate'];
 const events=[];let expansions=0,outputUnits=0;
 while(frames.length){
  const at=frames.length-4,file=readRequiredNumberFact(frames,at),next=readRequiredNumberFact(frames,at+1),offset=readRequiredNumberFact(frames,at+2),depth=readRequiredNumberFact(frames,at+3);
  frames=frames.slice(0,at);
  const directory=readRequiredTextFact(directories,directories.length-1),relative=readRequiredTextFact(relativeDirectories,relativeDirectories.length-1),owner=readRequiredTextFact(owners,owners.length-1);directories.pop();relativeDirectories.pop();owners.pop();
  const source=readRequiredTextFact(sources,file),path=readRequiredTextFact(paths,file),plan=numberColumn(plans,file),attributes=numberColumn(pathAttributes,file),tokens=textColumn(values,file),first=numberColumn(starts,file),last=numberColumn(ends,file);
  if(next===0&&offset===0)events.push('file',path,'0',String(source.length),'',owner,'');
  if(next*5>=plan.length){
   const intervals=readSourceIntervals(source,path,offset,source.length,owner,plan,attributes,tokens,first,last);
   for(let part=4;part<intervals.length;part+=7)outputUnits+=readRequiredTextFact(intervals,part).length;
   for(const part of intervals)events.push(part);
   if(outputUnits>outputUnitLimit)throw new RangeError('Rust project composed source unit limit exceeded');
   if(depth>0){if(outputUnits+2>outputUnitLimit)throw new RangeError('Rust project composed source unit limit exceeded');events.push('boundary',path,String(source.length),String(source.length),'\n}',owner,'');outputUnits+=2;}
   continue;
  }
  const keyword=readRequiredNumberFact(plan,next*5),name=readRequiredTextFact(tokens,readRequiredNumberFact(plan,next*5+1)),delimiter=readRequiredNumberFact(plan,next*5+2),ancestors=moduleAncestors(plan,tokens,next);
  const qualified=owner+'::'+(ancestors.length?ancestors.join('::')+'::':'')+name;
  if(depth+ancestors.length+1>depthLimit)throw new RangeError('Rust project module expansion limit exceeded');
  const context=moduleSourceContext(directory,relative,plan,attributes,tokens,next),literal=readRequiredNumberFact(attributes,next*3);
  const moduleDirectory=context.filter(part=>part!=='').join('/');
  const external=readRequiredTextFact(tokens,delimiter)===';',provider=external?(literal>=0?resolveRustProjectLiteralPath(readRequiredTextFact(context,0),readRequiredTextFact(tokens,literal)):selectRustProjectModuleSource(moduleDirectory,name,paths)):path;
  if(external&&!paths.includes(provider))throw new TypeError('missing explicit Rust project module file: '+provider);
  events.push('module',path,String(readRequiredNumberFact(first,keyword)),String(readRequiredNumberFact(last,readRequiredNumberFact(plan,next*5+3)-1)),'',qualified,provider);
  if(!external){frames.push(file,next+1,offset,depth);directories.push(directory);relativeDirectories.push(relative);owners.push(owner);continue;}
  expansions+=1;if(expansions>expansionLimit)throw new RangeError('Rust project module expansion limit exceeded');
  const beginning=readRequiredNumberFact(first,delimiter),after=readRequiredNumberFact(last,delimiter);
  if(beginning<offset||after<beginning||after>source.length)throw new TypeError('invalid Rust project module source range');
  const intervals=readSourceIntervals(source,path,offset,beginning,owner,plan,attributes,tokens,first,last);
  outputUnits+=2;
  for(let part=4;part<intervals.length;part+=7)outputUnits+=readRequiredTextFact(intervals,part).length;
  for(const part of intervals)events.push(part);
  if(outputUnits>outputUnitLimit)throw new RangeError('Rust project composed source unit limit exceeded');
  events.push('boundary',path,String(beginning),String(after),'{\n',qualified,provider);
  const child=paths.indexOf(provider);
  if(child===file)throw new TypeError('cyclic Rust project module file: '+provider);
  for(let active=0;active<frames.length;active+=4)if(readRequiredNumberFact(frames,active)===child)throw new TypeError('cyclic Rust project module file: '+provider);
  frames.push(file,next+1,after,depth);directories.push(directory);relativeDirectories.push(relative);owners.push(owner);
  frames.push(child,0,0,depth+ancestors.length+1);directories.push(readRustProjectModuleDirectory(provider,true));
  const filename=readRequiredTextFact(provider.split('/'),provider.split('/').length-1);
  relativeDirectories.push(literal>=0||filename==='mod.rs'?'':filename.slice(0,-3));owners.push(qualified);
 }
 return events;
}

/** Shared CLI transport plan: input, entry, target, binary, library. An empty
 * vector requests help; syntax failures retain the actual source TypeError.
 * @param {string[]} args @returns {string[]} */
export function readRustSourceProjectCommandPlan(args){
 if(args.length===0||args.length===1&&(readRequiredTextFact(args,0)==='help'||readRequiredTextFact(args,0)==='--help'))return [];
 const input=readRequiredTextFact(args,0);if(!input||input.startsWith('-'))throw new TypeError('expected a Links source inventory file');
 let entry='',target='',binary=false,library=false;
 for(let position=1;position<args.length;position+=1){
  const option=readRequiredTextFact(args,position);
  if(option==='--binary'){if(binary)throw new TypeError('duplicate --binary');binary=true;continue;}
  if(option==='--library'){if(library)throw new TypeError('duplicate --library');library=true;continue;}
  if((option!=='--entry'&&option!=='--to')||position+1>=args.length)throw new TypeError('invalid source project argument: '+option);
  const value=readRequiredTextFact(args,position+1);if(!value||value.startsWith('--'))throw new TypeError('missing source project option value: '+option);
  if(option==='--entry'){if(entry!=='')throw new TypeError('duplicate --entry');entry=value;}
  else{if(target!=='')throw new TypeError('duplicate --to');target=value;}
  position+=1;
 }
 if(entry==='')throw new TypeError('an explicit source project entry is required');
 if(binary&&target!=='')throw new TypeError('binary graph output and target source are separate representations');
 return [input,entry,target,binary?'true':'false',library?'true':'false'];
}
