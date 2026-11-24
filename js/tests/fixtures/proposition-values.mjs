// Independently authored literal inputs, not observations or generated output.
export const propositionValueDecisions=[
 {operation:'readPropositionValueForm',arguments:['bool'],expected:'value'},
 {operation:'readPropositionValueForm',arguments:['eq'],expected:'comparison'},
 {operation:'readPropositionValueForm',arguments:['forall'],expected:'finite quantification'},
 {operation:'readPropositionValueForm',arguments:['unknown'],expected:''},
 {operation:'readPropositionValueConditionalBranch',arguments:['and',true],expected:'right'},
 {operation:'readPropositionValueConditionalBranch',arguments:['and',false],expected:'false'},
 {operation:'readPropositionValueConditionalBranch',arguments:['or',true],expected:'true'},
 {operation:'readPropositionValueConditionalBranch',arguments:['or',false],expected:'right'},
 {operation:'readPropositionValueConditionalBranch',arguments:['implies',true],expected:'right'},
 {operation:'readPropositionValueConditionalBranch',arguments:['implies',false],expected:'true'},
 {operation:'planPropositionValueQuantifier',arguments:['bool'],expected:[0,1]},
 {operation:'planPropositionValueQuantifier',arguments:['unit'],expected:[0]},
 {operation:'planPropositionValueQuantifier',arguments:['nat'],expected:[]},
 {operation:'readPropositionValueDomainError',arguments:['string'],expected:'a proposition value requires an exhaustive Bool or Unit quantifier domain'},
 {operation:'readPropositionValueExpansionLimit',arguments:[],expected:256},
 {operation:'readPropositionValueQuantifierJoin',arguments:[],expected:'and'},
 {operation:'acceptPropositionValueComparison',arguments:['eq'],expected:true},
 {operation:'acceptPropositionValueComparison',arguments:['add'],expected:false},
 {operation:'renderRocqPropositionEqualityValue',arguments:['(left = right)',false],expected:'(if (ltac:(decide equality) : {(left = right)} + {~ ((left = right))}) then true else false)'},
 {operation:'renderRocqPropositionEqualityValue',arguments:['(left = right)',true],expected:'(negb (if (ltac:(decide equality) : {(left = right)} + {~ ((left = right))}) then true else false))'},
];
export const propositionValueSurfacePrograms=[
 {name:'exhaustive Bool equality retains each actual binder value',stdout:'true\n',program:{language:'Rust',items:[],main:{effects:[{k:'print',style:'rust',expr:{k:'propositionValue',prop:{p:'forall',binders:[{name:'flag',type:{kind:'bool'}}],body:{p:'eq',left:{k:'name',path:['flag']},right:{k:'name',path:['flag']}}}}}]}}},
 {name:'Unit quantifier is an exact singleton domain',stdout:'true\n',program:{language:'Rust',items:[],main:{effects:[{k:'print',style:'rust',expr:{k:'propositionValue',prop:{p:'forall',binders:[{name:'value',type:{kind:'unit'}}],body:{p:'eq',left:{k:'name',path:['value']},right:{k:'unit'}}}}}]}}},
 {name:'universal evaluation stops at its first false instance',stdout:'visited\nfalse\n',program:{language:'Rust',items:[],main:{effects:[{k:'print',style:'rust',expr:{k:'propositionValue',prop:{p:'forall',binders:[{name:'flag',type:{kind:'bool'}}],body:{p:'bool',expr:{k:'print',expr:{k:'str',value:'visited'},style:'rust',body:{k:'bool',value:false}}}}}}]}}},
 {name:'logical bridge does not evaluate an unselected effectful proposition',stdout:'true\n',program:{language:'Rust',items:[],main:{effects:[{k:'print',style:'rust',expr:{k:'propositionValue',prop:{p:'or',left:{p:'bool',expr:{k:'bool',value:true}},right:{p:'bool',expr:{k:'print',expr:{k:'str',value:'unselected'},style:'rust',body:{k:'bool',value:false}}}}}}]}}},
 {name:'atomic data equality evaluates its actual operands once in source order',stdout:'left\nright\ntrue\n',program:{language:'Rust',items:[{k:'data',name:'Number',ctors:[{name:'Number',fields:[{name:'number',type:{kind:'fixed',bits:8,signed:false}}]}]}],main:{effects:[{k:'print',style:'rust',expr:{k:'propositionValue',prop:{p:'eq',left:{k:'print',expr:{k:'str',value:'left'},style:'rust',body:{k:'app',fn:{k:'name',path:['Number','Number']},args:[{k:'num',value:'7',type:{kind:'fixed',bits:8,signed:false}}]}},right:{k:'print',expr:{k:'str',value:'right'},style:'rust',body:{k:'app',fn:{k:'name',path:['Number','Number']},args:[{k:'num',value:'7',type:{kind:'fixed',bits:8,signed:false}}]}}}}}]}}},
 {name:'SameValue keeps NaN equality distinct from ordinary numeric equality',stdout:'true\n',program:{language:'JavaScript',items:[],main:{effects:[{k:'print',style:'js-console',expr:{k:'propositionValue',prop:{p:'eq',sameValue:true,left:{k:'num',value:'NaN',type:{kind:'float'}},right:{k:'num',value:'NaN',type:{kind:'float'}}}}}]}}},
];
export const propositionValueSurfaceRefusals=[
 {name:'unbounded natural quantification is not finite runtime evidence',reasonIncludes:'exhaustive Bool or Unit',program:{language:'Rust',items:[],main:{effects:[{k:'print',style:'rust',expr:{k:'propositionValue',prop:{p:'forall',binders:[{name:'number',type:{kind:'nat'}}],body:{p:'bool',expr:{k:'bool',value:true}}}}}]}}},
 {name:'finite datatype quantification retains its unimplemented domain boundary',reasonIncludes:'exhaustive Bool or Unit',program:{language:'Rust',items:[{k:'data',name:'Choice',ctors:[{name:'First',fields:[]},{name:'Second',fields:[]}]}],main:{effects:[{k:'print',style:'rust',expr:{k:'propositionValue',prop:{p:'forall',binders:[{name:'choice',type:{kind:'named',path:['Choice']}}],body:{p:'bool',expr:{k:'bool',value:true}}}}}]}}},
 {name:'comparison bridge cannot invent an arithmetic proposition',reasonIncludes:'unknown proposition',program:{language:'Rust',items:[],main:{effects:[{k:'print',style:'rust',expr:{k:'propositionValue',prop:{p:'add',left:{k:'bool',value:true},right:{k:'bool',value:false}}}}]}}},
];
