// Literal policy inputs and original expectations; no plan or program executes.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/fixtures/javascript-string-arguments.lino',import.meta.url),{schemaVersion:1,model:'ordered JavaScript String argument capture 1',nodeVersionRange:'>=22.15 <23 or >=24 <25',rustVersionRange:'>=1.90 <1.100',rows:[
 {name:'the empty call retains a distinct result sentinel',count:0,expected:[-1]},
 {name:'a single operand retains its actual coordinate',count:1,expected:[0]},
 {name:'extra operands retain complete source evaluation order',count:3,expected:[0,1,2]},
 {name:'negative counts refuse',count:-1,expected:[]},
 {name:'fractional counts refuse',count:1.5,expected:[]},
 {name:'excessive captures refuse',count:4097,expected:[]},
 {name:'negative zero is not an argument coordinate',count:-0,expected:[]}
]});
