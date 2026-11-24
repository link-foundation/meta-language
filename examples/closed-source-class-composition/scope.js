// Selected complete Scope declaration from js/src/translation/ir.js. The
// complete original ir.js module remains a separate source obligation.
import { renderSourceTargetIdentifier } from '../../js/src/translation/source-identifier-rules.js';

export class Scope {
  constructor(ident, reserved = new Set()) {
    this.ident = ident;
    this.used = new Set(reserved);
  }

  fresh(name) {
    name = renderSourceTargetIdentifier(name);
    // A suffixed name is legal on its own: `end_2` needs none of the quoting of Lean's `«end»`.
    let candidate = renderSourceTargetIdentifier(this.ident(name));
    for (let index = 2; this.used.has(candidate); index += 1) candidate = renderSourceTargetIdentifier(this.ident(`${name}_${index}`));
    this.used.add(candidate);
    return candidate;
  }

  child() {
    const scope = new Scope(this.ident);
    scope.used = new Set(this.used);
    return scope;
  }
}
