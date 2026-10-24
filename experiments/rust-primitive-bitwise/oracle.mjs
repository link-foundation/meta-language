// Future paired CI producer; never execute during the complete code draft.
import { runInNewContext } from 'node:vm';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { structuredDataToLino } from '../../js/src/structured-data-links.js';
import { parseRust } from '../../js/src/translation/rust.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { emitJavaScript } from '../../js/src/translation/emit-javascript.js';
import { encodeTypedProgramLinks, emitTypedProgramLinks } from '../../js/src/typed-program-links.js';
const [major,minor]=process.versions.node.split('.').map(Number);
if (!(major===22&&minor>=15||major===24)) throw new RangeError('uncovered Node '+process.versions.node);
const fixture=readProjectData(new URL('../../parity/fixtures/rust-primitive-bitwise.lino',import.meta.url));
const source=fixture.source+fixture.precedenceSource+fixture.contextualSource;
const program=checkProgram(parseRust(source)), packet=encodeTypedProgramLinks(program), rust=emitRust(program), js=emitJavaScript(program);
const cases=[];
for(const row of fixture.functions){
  const type=fixture.types.find(type=>type.name===row.type);
  const name=js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.name).target;
  const fn=runInNewContext(js.text.replace(/^export /gmu,'')+'\n'+name+';');
  for(const left of type.values)for(const right of row.symbol==='!'?[type.values[0]]:type.values){
    const value=text=>type.name==='bool'?text==='true':BigInt(text);
    cases.push({name:row.name,type:type.name,left,right,expected:String(fn(value(left),value(right))),
      nativeTarget:rust.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.name).target});
  }
}
const header=rust.text.split('\n').find(line=>line.startsWith('#![allow('));
if(!header)throw new TypeError('the actual library attributes are required');
const eager=emitRust(checkProgram(parseRust(fixture.eagerSource)));
process.stdout.write(structuredDataToLino({source,cases,root:packet.root.asU64(),links:packet.network.toLino(),
  emissions:Object.fromEntries(['JavaScript','Rust','Lean','Rocq'].map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)])),
  nativeLibrary:[header,...rust.preludes,...rust.definitions].join('\n\n'),
  eagerSource:fixture.eagerSource,eagerLibrary:[...eager.preludes,...eager.definitions].join('\n\n')}));
