// Literal inputs and independent expected units only; no represented execution.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/fixtures/utf16-text-expansion.lino',import.meta.url),{
 schemaVersion:1,model:'bounded UTF16 repetition and padding 1',
 versions:{node:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rust:'>=1.90.0 <1.100.0; edition 2024'},
 repetition:[
  {text:[97,98],count:'2.9',length:4,expected:[97,98,97,98]},
  {text:[55296,0,56320],count:'2',length:6,expected:[55296,0,56320,55296,0,56320]},
  {text:[97],count:'NaN',length:0,expected:[]},
  {text:[97],count:'-0.9',length:0,expected:[]},
  {text:[],count:'1e100',length:0,expected:[]},
  {text:[],count:'Infinity',length:-1},
  {text:[97],count:'-Infinity',length:-1},
  {text:[97],count:'-1.9',length:-1},
  {text:[97],count:'1048576',length:1048576,expectedLength:1048576},
  {text:[97],count:'1048577',length:-2}
 ],
 padding:[
  {text:[97],filler:[120,121],maximum:'4.9',start:true,length:4,expected:[120,121,120,97]},
  {text:[97],filler:[120,121],maximum:'4.9',start:false,length:4,expected:[97,120,121,120]},
  {text:[97],filler:[55357,56832],maximum:'2',start:true,length:2,expected:[55357,97]},
  {text:[55296],filler:[0,56320],maximum:'4',start:false,length:4,expected:[55296,0,56320,0]},
  {text:[97,98],filler:[120],maximum:'NaN',start:true,length:2,expected:[97,98]},
  {text:[97,98],filler:[120],maximum:'-Infinity',start:false,length:2,expected:[97,98]},
  {text:[97],filler:[],maximum:'Infinity',start:true,length:1,expected:[97]},
  {text:[97],filler:[120],maximum:'Infinity',start:true,length:-2},
  {text:[],filler:[120],maximum:'1048576',start:false,length:1048576,expectedLength:1048576},
  {text:[97],filler:[120],maximum:'1048577',start:true,length:-2}
 ],
 boundary:'Unexecuted independent UTF16 controls. Negative plans distinguish invalid repetition counts from explicit resource refusal. Empty-string shortcuts remain semantic decisions, not skipped cells.'});
