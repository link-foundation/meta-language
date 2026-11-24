// Independently authored literal controls; not observations or executed evidence.
export const metadataCosts=[
 {name:'unit semantic projection',origin:{type:{kind:'unit'},path:[],wrappers:[],sourceOwner:{kind:'unit',path:[],arguments:[]}},expected:[20,203]},
 {name:'supplementary source path counts two UTF16 units',origin:{type:{kind:'unit'},path:['𐐀'],wrappers:[],sourceOwner:{kind:'unit',path:[],arguments:[]}},expected:[21,205]},
 {name:'expanded owner retains its own complete tree',origin:{type:{kind:'unit'},path:[],wrappers:[],sourceOwner:{kind:'unit',path:[],arguments:[]},expandedSourceOwner:{kind:'unit',path:[],arguments:[]}},expected:[23,224]},
];
export const reservations=[
 {input:[0,0,65536,0],expected:[1048576,0]},
 {input:[0,0,65537,0],expected:[]},
 {input:[0,0,0,524288],expected:[0,8388608]},
 {input:[0,0,0,524289],expected:[]},
 {input:[1048560,0,1,0],expected:[1048576,0]},
 {input:[1048560,0,2,0],expected:[]},
 {input:[0,8388592,0,1],expected:[0,8388608]},
 {input:[0,8388592,0,2],expected:[]},
];
