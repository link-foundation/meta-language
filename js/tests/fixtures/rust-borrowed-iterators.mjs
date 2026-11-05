// Literal expectations and Rust originals for future runs, never observations.
export const rustBorrowedIteratorDecisions=[
 {operation:'acceptRustBorrowedIteratorReceiver',arguments:[true,'iter',0,'array','library','std::vec::Vec',1,true],expected:true},
 {operation:'acceptRustBorrowedIteratorReceiver',arguments:[false,'iter',0,'array','library','std::vec::Vec',1,true],expected:false},
 {operation:'acceptRustBorrowedIteratorReceiver',arguments:[true,'into_iter',0,'array','library','std::vec::Vec',1,true],expected:false},
 {operation:'acceptRustBorrowedIteratorReceiver',arguments:[true,'iter',0,'array','declaration','std::vec::Vec',1,true],expected:false},
 {operation:'acceptRustBorrowedIteratorReceiver',arguments:[true,'iter',0,'array','library','std::vec::Vec',1,false],expected:false},
 {operation:'readRustBorrowedIteratorContinuation',arguments:['map'],expected:'map'},
 {operation:'readRustBorrowedIteratorContinuation',arguments:['copied'],expected:''},
 {operation:'readRustBorrowedIteratorProjectionSyntax',arguments:['match'],expected:'match'},
 {operation:'readRustBorrowedIteratorProjectionSyntax',arguments:['let'],expected:''},
 {operation:'readRustBorrowedIteratorProjectionSyntax',arguments:['app'],expected:''},
 {operation:'readRustBorrowedIteratorCallbackError',arguments:[true,1,true,true,true,0,true],expected:''},
 {operation:'readRustBorrowedIteratorCallbackError',arguments:[true,1,true,true,true,1,true],expected:'a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection'},
 {operation:'readRustBorrowedIteratorCallbackError',arguments:[true,1,true,false,true,0,true],expected:'a readonly iterator callback needs one actual immutable reference parameter, a closed reference result and a capture-free parameter projection'},
 {operation:'projectRustBorrowedIteratorCallbackRegions',arguments:[['parameter'],['receiver'],['parameter'],['receiver']],expected:['receiver']},
 {operation:'projectRustBorrowedIteratorCallbackRegions',arguments:[['receiver','parameter'],['receiver'],['parameter'],['receiver']],expected:['receiver']},
 {operation:'projectRustBorrowedIteratorCallbackRegions',arguments:[['old unrelated'],['old unrelated'],['old unrelated'],['receiver']],expected:[]},
 {operation:'projectRustBorrowedIteratorCallbackRegions',arguments:[['unknown'],[],['parameter'],['receiver']],expected:[]},
 {operation:'readRustBorrowedIteratorRegionError',arguments:[1,1,false],expected:''},
 {operation:'readRustBorrowedIteratorRegionError',arguments:[1,0,false],expected:'a readonly iterator value needs its actual live receiver-derived regions'},
 {operation:'acceptRustBorrowedIteratorOnce',arguments:['std::iter::once',false,1],expected:true},
 {operation:'acceptRustBorrowedIteratorOnce',arguments:['std::iter::once',true,1],expected:false},
 {operation:'acceptRustBorrowedIteratorOnce',arguments:['my::once',false,1],expected:false},
 {operation:'acceptRustBorrowedIteratorBoxReference',arguments:[true,true,0,true,true,false],expected:true},
 {operation:'acceptRustBorrowedIteratorBoxReference',arguments:[true,false,0,true,true,false],expected:false},
 {operation:'acceptRustBorrowedIteratorBoxReference',arguments:[true,true,0,true,false,false],expected:false},
 {operation:'readRustBorrowedIteratorBudgetError',arguments:[64,100000,65536],expected:''},
 {operation:'readRustBorrowedIteratorBudgetError',arguments:[65,0,0],expected:'the readonly iterator source pipeline exceeds its finite profile'},
 {operation:'readRustBorrowedIteratorBudgetError',arguments:[0,NaN,0],expected:'invalid readonly iterator source resource count'},
 {operation:'readRustBorrowedIteratorBudgetError',arguments:[0,0,-0],expected:'invalid readonly iterator source resource count'},
 {operation:'readRustBorrowedIteratorBudgetError',arguments:[0,Infinity,0],expected:'invalid readonly iterator source resource count'},
 {operation:'readRustBorrowedIteratorBudgetError',arguments:[0,0.5,0],expected:'invalid readonly iterator source resource count'},
 {operation:'readRustBorrowedIteratorBudgetError',arguments:[0,-1,0],expected:'invalid readonly iterator source resource count'},
 {operation:'readRustBorrowedIteratorBudgetError',arguments:[0,9007199254740992,0],expected:'invalid readonly iterator source resource count'},
 {operation:'acceptRustBorrowedIteratorProduction',arguments:['map',true,true,true],expected:true},
 {operation:'acceptRustBorrowedIteratorProduction',arguments:['map',true,true,false],expected:false},
 {operation:'acceptRustBorrowedIteratorWholeSlice',arguments:[['lit','lit'],['float','float'],['0','Infinity']],expected:true},
 {operation:'acceptRustBorrowedIteratorWholeSlice',arguments:[['lit','lit'],['float','float'],['1','Infinity']],expected:false},
 {operation:'acceptRustBorrowedIteratorWholeSlice',arguments:[['lit','lit'],['fixed','float'],['0','Infinity']],expected:false},
 {operation:'readRustBorrowedIteratorSnapshotError',arguments:[100000,16777216],expected:''},
 {operation:'readRustBorrowedIteratorSnapshotError',arguments:[100001,0],expected:'readonly callback ownership snapshots exceed their finite source profile'},
 {operation:'readRustBorrowedIteratorSnapshotError',arguments:[0,-0],expected:'readonly callback ownership snapshots exceed their finite source profile'},
 {operation:'readRustBorrowedIteratorParameterError',arguments:['field','field'],expected:''},
 {operation:'readRustBorrowedIteratorParameterError',arguments:['mut','mut'],expected:'the readonly callback needs a fresh direct immutable binding identifier'},
 {operation:'readRustBorrowedIteratorParameterError',arguments:['mut','r#mut'],expected:''},
 {operation:'readRustBorrowedIteratorBindingOwnerError',arguments:['ctor',false],expected:'readonly callback parameters require a fresh binder rather than a constructor or constant pattern'},
 {operation:'readRustBorrowedIteratorBindingOwnerError',arguments:['',true],expected:'readonly callback parameters require a fresh binder rather than a constructor or constant pattern'},
 {operation:'readRustBorrowedIteratorBindingOwnerError',arguments:['fn',false],expected:''},
 {operation:'readRustBorrowedIteratorRegionWorkError',arguments:[256],expected:''},
 {operation:'readRustBorrowedIteratorRegionWorkError',arguments:[257],expected:'readonly iterator region union exceeds its bounded source work profile'},
 {operation:'readRustBorrowedIteratorRegionWorkError',arguments:[-0],expected:'readonly iterator region union exceeds its bounded source work profile'},
 {operation:'readRustBorrowedIteratorRegionWorkError',arguments:[NaN],expected:'readonly iterator region union exceeds its bounded source work profile'},
 {operation:'readRustBorrowedIteratorBoxReferenceError',arguments:[true,1,false,false],expected:'canonical Box::as_ref expects zero arguments'},
 {operation:'readRustBorrowedIteratorBoxReferenceError',arguments:[true,0,false,false],expected:'canonical Box::as_ref requires its actual live readonly receiver projection'},
 {operation:'readRustBorrowedIteratorBoxReferenceError',arguments:[false,1,false,false],expected:''},
];
export const rustBorrowedIteratorPendingOriginals=[
 {name:'readonly collection retains borrowed nominal vector elements without source Clone',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',stdout:'1\n',source:"struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a Node>{values.iter().collect()}fn main(){let values=vec![Node(7u8)];println!(\"{}\",children(&values).len());}"},
 {name:'reference projection maps actual nominal fields without captures or source Clone',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',stdout:'1\n',source:"struct Node(u8);struct Field{ty:Node}fn children<'a>(fields:&'a Vec<Field>)->Vec<&'a Node>{fields.iter().map(|field|&field.ty).collect()}fn main(){let fields=vec![Field{ty:Node(7u8)}];println!(\"{}\",children(&fields).len());}"},
 {name:'readonly chain authentic once and Box as_ref preserve actual source order',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',stdout:'2\n',source:"struct Node(u8);fn children<'a>(values:&'a Vec<Node>,tail:&'a Box<Node>)->Vec<&'a Node>{values.iter().chain(std::iter::once(tail.as_ref())).collect()}fn main(){let values=vec![Node(7u8)];let tail=Box::new(Node(8u8));println!(\"{}\",children(&values,&tail).len());}"},
 {name:'readonly collect retains its explicit actual Vec reference element inference',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',stdout:'1\n',source:"struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a Node>{values.iter().collect::<Vec<_>>()}fn main(){let values=vec![Node(7u8)];println!(\"{}\",children(&values).len());}"},
 {name:'raw callback binding identifiers retain their actual lexical binding role',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',stdout:'1\n',source:"struct Node(u8);struct Field{ty:Node}fn children<'a>(fields:&'a Vec<Field>)->Vec<&'a Node>{fields.iter().map(|r#mut|&r#mut.ty).collect()}fn main(){let fields=vec![Field{ty:Node(7u8)}];println!(\"{}\",children(&fields).len());}"},
];
// The method body below is retained from rust/src/translation/types.rs,
// Type::children. The local datatype inventory supplies its actual dependencies;
// it is not a claim that the entire project source is already admitted.
export const rustTypeChildrenOriginalMethod=`pub fn children(&self) -> Vec<&Self> {
        match self {
            Self::Array { element, .. }
            | Self::Option { element, .. }
            | Self::Set { element, .. }
            | Self::Cell { element } => {
                vec![element]
            }
            Self::Tuple { elements } => elements.iter().collect(),
            Self::Union { alternatives } => alternatives.iter().collect(),
            Self::Record { fields, .. } => fields.iter().map(|field| &field.ty).collect(),
            Self::Map { key, value, .. } => vec![key, value],
            Self::Function { params, ret } => {
                params.iter().chain(std::iter::once(ret.as_ref())).collect()
            }
            _ => Vec::new(),
        }
    }`;
export const rustTypeChildrenPendingOriginal={name:'actual own Type children method retains immediate child order and references',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',stdout:'1\n2\n0\n',source:`enum Type{Array{element:Box<Self>},Option{element:Box<Self>},Set{element:Box<Self>},Cell{element:Box<Self>},Tuple{elements:Vec<Self>},Union{alternatives:Vec<Self>},Record{fields:Vec<TypeField>},Map{key:Box<Self>,value:Box<Self>},Function{params:Vec<Self>,ret:Box<Self>},Leaf}struct TypeField{ty:Type}impl Type{${rustTypeChildrenOriginalMethod}}fn main(){let record=Type::Record{fields:vec![TypeField{ty:Type::Leaf}]};println!("{}",record.children().len());let function=Type::Function{params:vec![Type::Leaf],ret:Box::new(Type::Leaf)};println!("{}",function.children().len());let leaf=Type::Leaf;println!("{}",leaf.children().len());}`};
export const rustBorrowedIteratorRefusals=[
 {name:'a reference callback cannot return a newly constructed local referent',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',originalCompilerOutcome:'reject',source:"struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a Node>{values.iter().map(|value|{let local=Node(1u8);&local}).collect()}fn main(){}"},
 {name:'captured reference callbacks retain their separate unsupported boundary',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',originalCompilerOutcome:'accept',source:"struct Node(u8);fn children<'a>(values:&'a Vec<Node>,extra:&'a Node)->Vec<&'a Node>{values.iter().map(|value|extra).collect()}fn main(){}"},
 {name:'a borrowed vector cannot outlive a subsequent move of its actual source',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',originalCompilerOutcome:'reject',source:"struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a Node>{values.iter().collect()}fn main(){let values=vec![Node(7u8)];let refs=children(&values);let moved=values;println!(\"{}\",refs.len());}"},
 {name:'a readonly collector cannot change the actual nominal child owner',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',originalCompilerOutcome:'reject',source:"struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a u8>{values.iter().collect()}fn main(){}"},
 {name:'local std modules cannot masquerade as the actual standard once function',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',originalCompilerOutcome:'accept',source:"struct Node(u8);mod std{pub mod iter{pub fn once(value:&super::super::Node)->Vec<&super::super::Node>{vec![value]}}}fn children<'a>(values:&'a Vec<Node>,tail:&'a Node)->Vec<&'a Node>{values.iter().chain(std::iter::once(tail)).collect()}fn main(){}"},
 {name:'unit constructor patterns cannot be silently reinterpreted as fresh callback binders',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',originalCompilerOutcome:'reject',source:"struct Marker;struct Node(u8);fn children<'a>(values:&'a Vec<Node>)->Vec<&'a Node>{values.iter().map(|Marker|Marker).collect()}fn main(){}"},
 {name:'wrong Box as_ref arity cannot fall through to an inner nominal method',sourceVersionRange:'>=1.90.0 <1.100.0',sourceSignatureModel:'Rust source function signature 3',originalCompilerOutcome:'reject',source:'struct Node(u8);impl Node{fn as_ref(&self,value:u8)->u8{value}}fn main(){let boxed=Box::new(Node(7u8));let value=boxed.as_ref(9u8);println!("{}",value);}'},
];
