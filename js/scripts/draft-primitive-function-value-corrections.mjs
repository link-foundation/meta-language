// Documentary assertion correction only; original inputs move into a complete
// positive profile with native products. Every preceding fixture stays hashed.
export function draftPrimitiveFunctionValueCorrections(data,fixture){
 const name=fixture==='rust-primitive-associated-call'?'associated function values retain a separate callable obligation':fixture==='rust-primitive-default'?'a primitive Default function value retains its callable obligation':fixture==='rust-primitive-from'?'a conversion function value retains a separate callable obligation':null;
 if(!name)return data;
 const retained=data.refusals.find(row=>row.name===name);if(!retained)return data;
 data.refusals=data.refusals.filter(row=>row!==retained);
 data.assertionCorrections??=[];
 const revision=fixture==='rust-primitive-from'?'7f1cf100':'59341c5f',destination=fixture==='rust-primitive-from'?'rust-contextual-from-function-value':'rust-primitive-function-value';
 data.assertionCorrections.push({input:retained.source,priorName:retained.name,priorKind:retained.kind,priorReason:retained.reason,after:'supported scalar function value; exact original input retained as a positive native-product control',fixture:'parity/fixtures/'+destination+'.lino',retainedFixture:'parity/fixtures/history/'+fixture+'-'+revision+'.lino',sourceRevision:revision,versions:'Rust >=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',reference:'https://doc.rust-lang.org/reference/types/function-item.html#function-item-types'});
 return data;
}
