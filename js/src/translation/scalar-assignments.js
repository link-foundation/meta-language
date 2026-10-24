// Mechanical source-node construction from the complete shared term plan.
import { planScalarAssignmentExpression, readScalarAssignmentOperation } from './scalar-assignment-rules.js';
import { unsupported } from './diagnostics.js';

export function constructScalarAssignmentExpression(cell, right, operator, postfix, names, span) {
  const plan = planScalarAssignmentExpression(operator,postfix), operation = readScalarAssignmentOperation(operator), nodes = [];
  if (!plan.length || names.length !== 2) throw unsupported('scalar assignment expression','this operator needs its complete source operation',span);
  const actual = position => {
    if (!Number.isSafeInteger(position) || position < 0 || position >= nodes.length) throw new TypeError('a scalar assignment plan needs an earlier actual source term');
    return nodes[position];
  };
  for (let at=0;at<plan.length;at+=4) {
    const [code,first,second,third] = plan.slice(at,at+4);
    let node;
    if (code === 0) node = first === 0 ? cell : right;
    else if (code === 1) node = {k:'name',path:[names[first]],span};
    else if (code === 2) node = {k:'cellRead',cell:actual(first),span};
    else if (code === 3) node = {k:'num',value:String(first),unit:true,span};
    else if (code === 4) node = {k:'binary',op:operation,left:actual(second),right:actual(third),span};
    else if (code === 5) node = {k:'math',op:operation,name:operator,args:[{value:actual(second),spread:false},{value:actual(third),spread:false}],span};
    else if (code === 6) node = {k:'coalesce',left:actual(first),right:actual(second),span};
    else if (code === 7) node = {k:'let',name:names[first],value:actual(second),body:actual(third),span};
    else if (code === 8) node = {k:'cellWrite',cell:actual(first),value:actual(second),body:actual(third),span};
    else if (code === 9) node = {k:'if',cond:actual(first),then:actual(second),else:actual(third),span};
    else if (code === 10) node = {k:'unary',op:'neg',arg:actual(first),span};
    else throw new TypeError('unknown scalar assignment construction operation');
    if (!node) throw new TypeError('scalar assignment source transport lost an actual operand');
    nodes.push(node);
  }
  return nodes.at(-1);
}
