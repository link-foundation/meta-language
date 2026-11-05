// Checked declaration ownership/pattern coordinates into the shared renderer.
import {acceptNominalArrayProjectionLoan,renderNominalArrayProjectionLoan} from './nominal-array-loan-rules.js';
import {renderRustConstructorFields} from './rust-record-rules.js';
export function emitNominalArrayProjectionLoan(emitter,expression){
  if(expression.k!=='match'||expression.scrutinee.type.kind!=='data')return null;
  const entry=emitter.program.declarations.get(expression.scrutinee.type.name),kase=expression.cases[0];
  const pattern=kase?.pattern,ctor=entry?.k==='data'?entry.ctors[0]:undefined;
  const name=kase?.body.k==='var'?kase.body.name:'';
  const bindings=pattern?.k==='ctor'?pattern.binds:[],slot=bindings.indexOf(name);
  const direct=kase?.body.k==='var'&&ctor?.fields[slot]?.type.kind==='array'&&!ctor.fields[slot].type.reference;
  if(!acceptNominalArrayProjectionLoan(expression.type.kind,expression.type.reference===true,expression.scrutinee.k,
    entry?.ctors?.length??0,expression.cases.length,pattern?.k==='ctor'&&pattern.data===entry?.fullName&&pattern.ctor===ctor?.name,
    ctor?.fields.length??0,bindings.length,slot,bindings.filter(binding=>binding===name).length,direct===true))return null;
  const head='crate::'+emitter.state.ctorRef(pattern.data,pattern.ctor,'::');
  const rendered=renderRustConstructorFields(emitter.program.sourceLanguage,head,ctor.fields.map(field=>field.name??''),bindings.map(binding=>binding??'_'));
  return renderNominalArrayProjectionLoan(expression.scrutinee.name,rendered,name);
}
