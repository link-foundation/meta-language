// Independent complete sources and scalar observations for the explicit target profile.
export const declarationScalarLoopModel='closed declaration scalar loop 1';
export const declarationScalarLoopCases=[
 {language:'Rust',name:'run',source:'fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{run};}}fn main(){}',observations:[[[0,2],2],[[0,20000],20000],[[NaN,2],NaN],[[-0,-1],-0]]},
 {language:'JavaScript',name:'run',source:'/** @param {number} run @param {number} limit @returns {number} */function run(run,limit){for(;;){if(run<limit){const ml_scalar_write0=run+1;[run,limit]=[ml_scalar_write0,limit];continue;}else{return run;}}}',observations:[[[0,2],2],[[0,20000],20000],[[NaN,2],NaN],[[-0,-1],-0]]},
 {language:'Rust',name:'count',source:'fn count(mut value:f64,limit:f64,mut at:f64)->f64{loop{return if at<limit{let count=value+1f64;(value,at)=(count,at+1f64);continue;}else{value};}}fn main(){}',observations:[[[3,5,0],8],[[NaN,2,0],NaN],[[-0,0,0],-0]]},
 {language:'JavaScript',name:'count',source:'/** @param {number} value @param {number} limit @param {number} at @returns {number} */function count(value,limit,at){for(;;){if(at<limit){const count=value+1;const ml_scalar_write0=count;const ml_scalar_write1=at+1;[value,limit,at]=[ml_scalar_write0,limit,ml_scalar_write1];continue;}else{return value;}}}',observations:[[[3,5,0],8],[[NaN,2,0],NaN],[[-0,0,0],-0]]},
 {language:'Rust',name:'tick',source:'fn tick(mut tick:f64)->f64{loop{return if tick<2f64{(tick,)=(tick+1f64,);continue;}else{tick};}}fn main(){}',observations:[[[0],2],[[NaN],NaN],[[Infinity],Infinity]]},
 {language:'JavaScript',name:'tick',source:'/** @param {number} tick @returns {number} */function tick(tick){for(;;){if(tick<2){const ml_scalar_write0=tick+1;tick=ml_scalar_write0;continue;}else{return tick;}}}',observations:[[[0],2],[[NaN],NaN],[[Infinity],Infinity]]},
];
