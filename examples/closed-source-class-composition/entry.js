// Complete supplied module workload. Import aliases are compiler-resolved;
// no fixture hand-renames constructors, self references or helper symbols.
import { NameScope } from './public-scope.js';
/** @param {string} name @returns {string} */
function identity(name) { return name; }
/** @param {string} name @returns {string} */
export function run(name) {
  const root = new NameScope(identity);
  const first = root.fresh(name);
  const child = root.child();
  return first + '/' + child.fresh(name) + '/' + root.fresh(name);
}
