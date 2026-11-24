// Independent literal target syntax/resource expectations; no observations.
export const formalDatatypeAliasDecisions=[
 {operation:'renderFormalDatatypeTypeAlias',arguments:['Lean','Public','private.Scope'],expected:'abbrev Public := _root_.private.Scope'},
 {operation:'renderFormalDatatypeTypeAlias',arguments:['Rocq','Public','private.Scope'],expected:'Definition Public : Type := private.Scope.'},
 {operation:'renderFormalDatatypeConstructorAlias',arguments:['Lean','Public','More','private.Scope.More'],expected:'namespace Public\n@[match_pattern] abbrev More := _root_.private.Scope.More\nend Public'},
 {operation:'renderFormalDatatypeConstructorAlias',arguments:['Rocq','Public','More','private.More'],expected:'Notation More := private.More.'},
 {operation:'renderFormalDatatypeAliasModule',arguments:['Lean','inner',true],expected:'namespace inner'},
 {operation:'renderFormalDatatypeAliasModule',arguments:['Lean','inner',false],expected:'end inner'},
 {operation:'renderFormalDatatypeAliasModule',arguments:['Rocq','inner',true],expected:'Module inner.'},
 {operation:'renderFormalDatatypeAliasModule',arguments:['Rocq','inner',false],expected:'End inner.'},
 {operation:'renderFormalDatatypeTypeAlias',arguments:['unknown','Public','Value'],expected:''},
 {operation:'readFormalDatatypeAliasBudgetError',arguments:[8192,16777216],expected:''},
 {operation:'readFormalDatatypeAliasBudgetError',arguments:[8193,0],expected:'formal datatype public aliases exceed their finite target budget'},
 {operation:'readFormalDatatypeAliasBudgetError',arguments:[0,16777217],expected:'formal datatype public aliases exceed their finite target budget'},
 {operation:'readFormalDatatypeAliasBudgetError',arguments:[NaN,0],expected:'formal datatype public aliases exceed their finite target budget'},
 {operation:'readFormalDatatypeAliasBudgetError',arguments:[0,Infinity],expected:'formal datatype public aliases exceed their finite target budget'},
 {operation:'readFormalDatatypeAliasBudgetError',arguments:[-1,0],expected:'formal datatype public aliases exceed their finite target budget'},
 {operation:'readFormalDatatypeAliasBudgetError',arguments:[0,0.5],expected:'formal datatype public aliases exceed their finite target budget'},
 {operation:'readFormalDatatypeAliasTraversalError',arguments:[1048576,256],expected:''},
 {operation:'readFormalDatatypeAliasTraversalError',arguments:[1048577,0],expected:'formal datatype alias scope traversal exceeds its finite target budget'},
 {operation:'readFormalDatatypeAliasTraversalError',arguments:[0,257],expected:'formal datatype alias scope traversal exceeds its finite target budget'},
 {operation:'readFormalDatatypeAliasTraversalError',arguments:[NaN,0],expected:'formal datatype alias scope traversal exceeds its finite target budget'},
 {operation:'readFormalDatatypeAliasTraversalError',arguments:[0,-1],expected:'formal datatype alias scope traversal exceeds its finite target budget'},
 {operation:'readFormalDatatypeAliasTraversalError',arguments:[0,1.5],expected:'formal datatype alias scope traversal exceeds its finite target budget'},
];
export const formalDatatypeAliasSurfaces=[
 {name:'actual Expr and nested Node preserve public constructor pattern aliases at one module opening',surface:{language:'Rust',items:[{k:'module',name:'ir',items:[
  {k:'data',name:'Expr',ctors:[{name:'Value',fields:[{name:'node',type:{kind:'data',name:'ir.nodes.Node'}}]}]},
  {k:'module',name:'nodes',items:[{k:'data',name:'Node',ctors:[{name:'Unit',fields:[]},{name:'Unary',fields:[{name:'argument',type:{kind:'data',name:'ir.Expr'}}]}]}]},
 ]}]}},
 {name:'whole protected datatype dependencies and unrelated functions wait for lifted blocks',surface:{language:'Rust',items:[{k:'module',name:'left',items:[
  {k:'fn',name:'available',params:[],ret:{kind:'bool'},body:{k:'bool',value:true}},
  {k:'data',name:'Dependency',ctors:[{name:'Value',fields:[]}]},
  {k:'data',name:'First',ctors:[{name:'End',fields:[]},{name:'More',fields:[{name:'other',type:{kind:'data',name:'right.Second'}},{name:'dependency',type:{kind:'data',name:'left.Dependency'}}]}]},
 ]},{k:'module',name:'right',items:[{k:'data',name:'Second',ctors:[{name:'Wrap',fields:[{name:'value',type:{kind:'data',name:'left.First'}}]}]}]}]}},
];
