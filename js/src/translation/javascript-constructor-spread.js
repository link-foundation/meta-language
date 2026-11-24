import { typeError, unsupported } from './diagnostics.js';
import { readJavaScriptConstructorTagAlternativeLimit } from './javascript-surface-rules.js';

/** Lower copies from finite literal records into ordinary constructor arms.
 * Binding the already visited properties at each decision preserves source
 * evaluation order even when the selected constructor fields differ. */
export function expandJavaScriptConstructorSpreads(tag, entries, freshName, where) {
  const limit = readJavaScriptConstructorTagAlternativeLimit();
  const refuse = (reason, span = where) => { throw unsupported('tagged object spread', reason, span); };
  if (tag.k !== 'str') refuse('literal record spreads currently require a single literal tag', tag.span);

  function prepare(entries) {
    let count = 1;
    const steps = [];
    for (const entry of entries) {
      if (!entry.spread) {
        const index = Number(entry.name);
        if (String(index) === entry.name && Number.isInteger(index) && index >= 0 && index < 4294967295) {
          refuse('constructor spreads need named fields with a stable declaration order', entry.value.span);
        }
        steps.push(entry);
        continue;
      }
      const spread = prepareOperand(entry.value);
      steps.push(...spread.steps);
      count *= spread.count;
      if (count > limit) refuse('literal record spreads exceed the finite constructor alternative limit', entry.value.span);
    }
    return { steps, count };
  }

  function prepareOperand(value) {
    if (value.k === 'record' && !value.prototypeDefinition) return prepare(value.entries);
    if (value.k !== 'if') refuse('constructor spreads require a complete tagged field projection from literal records', value.span);
    const yes = prepareOperand(value.then), no = prepareOperand(value.else);
    const count = yes.count + no.count;
    if (count > limit) refuse('literal record spreads exceed the finite constructor alternative limit', value.span);
    return {steps: [{spread: true, condition: value.cond, then: yes.steps, else: no.steps}], count};
  }

  function lower(steps, fields, bound) {
    for (const [index, step] of steps.entries()) {
      if (!step.spread) {
        if (Object.hasOwn(fields, step.name)) throw typeError(`duplicate field ${step.name}`, step.value.span ?? where);
        fields[step.name] = step.value;
        continue;
      }
      const bindings = Object.entries(fields).slice(bound).map(([name, value]) => {
        const binding = freshName();
        fields[name] = { k: 'name', path: [binding], span: value.span };
        return { name: binding, value };
      });
      const rest = steps.slice(index + 1), nextBound = bound + bindings.length;
      const branch = arm => lower(arm.concat(rest), Object.assign(Object.create(null), fields), nextBound);
      const value = { k: 'if', cond: step.condition, then: branch(step.then), else: branch(step.else), span: where };
      return bindings.reduceRight((body, binding) => ({ k: 'let', ...binding, body, span: where }), value);
    }
    return { k: 'ctorObject', tag: tag.value, fields, span: where };
  }

  return lower(prepare(entries).steps, Object.create(null), 0);
}
