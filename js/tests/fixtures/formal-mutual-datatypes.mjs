// Complete independent literal controls owned by callable-data derivation rules.
export const formalMutualDatatypeDecisions=[
 {operation:'readFormalDatatypeConstructivePlan',arguments:[['reference','constructor','data','constructor','reference','constructor','data','reference'],[0,1,2,2,2,3,5,5],[0,1,4,3,5],[1,-1,-1,-1,0,-1,-1,0],[2,6],7],expected:['witnesses','0','2','0','1']},
 {operation:'readFormalDatatypeConstructivePlan',arguments:[['reference','constructor','data','reference','constructor','data','reference'],[0,1,2,2,3,4,4],[0,1,3,4],[1,-1,-1,0,-1,-1,0],[2,5],6],expected:['error','callable data has no finite constructive witness for every referenced datatype']},
 {operation:'readFormalDatatypeConstructivePlan',arguments:[['reference','bool','function','constructor','constructor','data','reference'],[0,0,2,3,3,5,5],[0,1,2,3,4],[0,-1,-1,-1,-1,-1,0],[5],6],expected:['error','recursive callable data in a function domain lacks strict positivity']},
 {operation:'readFormalDatatypeConstructivePlan',arguments:[['reference'],[0],[],[0],[0],0],expected:['error','incomplete callable data declaration roots']},
 {operation:'readFormalDatatypeLeanBlockDerivation',arguments:[false,false,false],expected:'Repr, DecidableEq'},
 {operation:'readFormalDatatypeLeanBlockDerivation',arguments:[false,false,true],expected:'Repr'},
 {operation:'readFormalDatatypeLeanBlockDerivation',arguments:[true,false,false],expected:''},
 {operation:'readFormalDatatypeLeanBlockDerivation',arguments:[false,true,false],expected:''},
];
export const formalMutualDatatypeSurfaces=[
 {name:'finite mutual defaults follow the real constructor dependency rather than source order',surface:{language:'Rust',items:[
  {k:'data',name:'First',ctors:[{name:'Wrap',fields:[{name:'value',type:{kind:'data',name:'Second'}}]}]},
  {k:'data',name:'Second',ctors:[{name:'End',fields:[]},{name:'More',fields:[{name:'value',type:{kind:'data',name:'First'}}]}]},
 ]}},
];
