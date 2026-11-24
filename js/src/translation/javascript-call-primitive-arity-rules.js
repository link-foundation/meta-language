import {readJavaScriptSourceExceptionClass} from './source-exception-rules.js';

/** @param {string} operation @returns {number} */
export function readJavaScriptCallIntegerArity(operation){
 if(operation==='negate integer'||operation==='complement integer bits'||operation==='complement number bits')return 1;
 return ['convert integer radix text','wrap signed integer','wrap unsigned integer','add integers','subtract integers','multiply integers','divide integers','read integer remainder','raise integer power','compare integers','compare number integer','compare text integer',
 'conjoin integer bits','disjoin integer bits','exclude equal integer bits','shift integer bits left','shift integer bits right',
 'conjoin number bits','disjoin number bits','exclude equal number bits','shift number bits left','shift number bits right','shift number bits right unsigned'].includes(operation)?2:0;
}

/** @param {string} operation @returns {number} */
export function readJavaScriptNumericParsingArity(operation){return operation==='parse integer text prefix'?2:operation==='parse float text prefix'?1:-1;}

/** @param {string} operation @returns {number} */
export function readJavaScriptStringMethodPrimitiveArity(operation){return ['test well formed String','copy well formed String','convert character code text','convert code point text'].includes(operation)?1:['map lower String case','map upper String case','require String receiver','read String code unit','test String own key','read String own key'].includes(operation)?2:['substring String units','substr String units','test String prefix at','test String suffix at','test String inclusion at'].includes(operation)?3:-1;}

/** @param {string} operation @returns {boolean} */
export function testJavaScriptCallUnaryNumberOperation(operation){
 return ['test NaN number','test finite number','negate number','truncate number','read absolute number','floor number','ceil number','round number','read number sign','read square root','test integer number','test safe integer number'].includes(operation);
}
/** Operations consume already checked primitive values. Unmodeled coercions,
 * wrappers and replacement intrinsics need their own source lowering.
 * @param {string} operation @param {number} count @returns {string} */
export function readJavaScriptCallPrimitiveArityError(operation,count){
 const parsingArity=readJavaScriptNumericParsingArity(operation);if(parsingArity>=0)return count===parsingArity?'':'numeric parsing retains every converted operand';
 const stringArity=readJavaScriptStringMethodPrimitiveArity(operation);if(stringArity>=0)return count===stringArity?'':'String operations retain every actual converted operand';
 if(!Number.isSafeInteger(count)||count<0||count>4096)return 'primitive operands exceed their explicit call register budget';
 if(operation==='throw restricted Function access')return count===0?'':'restricted Function access ignores receiver and arguments';
 if(readJavaScriptSourceExceptionClass(operation)!=='')return count===2?'':'a source Error requires its actual text message and own-message presence';
 const integerArity=readJavaScriptCallIntegerArity(operation);
 if(integerArity!==0)return count===integerArity?'':'primitive instructions require every actual operand coordinate';
 if(operation==='read minimum number'||operation==='read maximum number'||operation==='construct text units')return '';
 if(testJavaScriptCallUnaryNumberOperation(operation))return count===1?'':'primitive instructions require every actual operand coordinate';
 const unary=['require Array creation length','require V8 array join length','construct Symbol descriptive text','round binary32 number','count leading zero bits','read integer index','require callable target','require array like arguments Object','require Object assign target','require Object reflection target','require nonnull Object input','require object for in','require ordinary property object','require array method object','read array like length','require delete success','require collection adder','require Map entry object','require object coercible','test object value','read semantic type','read text length','trim text','trim text start','trim text end','convert explicit number','coerce primitive number','coerce primitive numeric','convert primitive value','convert number primitive value','increment numeric value','decrement numeric value','negate numeric value','convert primitive text','convert explicit integer','coerce primitive integer'];
 const binary=['read array deletion count','read checked array replacement index','multiply signed integer bits','read relative array index','read checked array append length','compare relational values','subtract numeric values','multiply numeric values','divide numeric values','read numeric remainder','raise numeric power','repeat text','read text code point','read text character','read relative text character','concatenate text','compare text','test text prefix','test text suffix','test text inclusion','add primitive values'];
 const ternary=['pad text start','pad text end','slice text','find text','find last text','replace text','replace all text'];
 const arity=unary.includes(operation)?1:binary.includes(operation)?2:ternary.includes(operation)?3:-1;
 return arity<0?'primitive instructions require their actual registered operation':count!==arity?'primitive instructions require every actual operand coordinate':'';
}
