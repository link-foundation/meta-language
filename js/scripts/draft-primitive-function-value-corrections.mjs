// Documentary assertion correction only; original inputs move into a complete
// positive profile with native products. Every preceding fixture stays hashed.
export function draftPrimitiveFunctionValueCorrections(data,fixture){
 const name=fixture==='rust-primitive-associated-call'?'associated function values retain a separate callable obligation':fixture==='rust-primitive-default'?'a primitive Default function value retains its callable obligation':null;
 if(!name)return data;
 const retained=data.refusals.find(row=>row.name===name);if(!retained)return data;
 data.refusals=data.refusals.filter(row=>row!==retained);
 data.assertionCorrections??=[];
 data.assertionCorrections.push({input:retained.source,priorName:retained.name,priorKind:retained.kind,priorReason:retained.reason,after:'supported scalar function value; exact original input retained as a positive native-product control',fixture:'parity/fixtures/rust-primitive-function-value.lino',retainedFixture:'parity/fixtures/history/'+fixture+'-59341c5f.lino',sourceRevision:'59341c5f',versions:'Rust >=1.90.0 <1.100.0; edition 2024; target_pointer_width=64',reference:'https://doc.rust-lang.org/reference/types/function-item.html#function-item-types'});
 return data;
}
