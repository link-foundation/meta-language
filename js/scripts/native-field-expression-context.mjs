// Recorded source contexts restrict operands while keeping source aliases,
// precedence, fields and explicitly grouped expressions in the original rules.
export function transformFieldExpressionContext(grammar, { clones, fields }) {
  const existing = name => typeof name === 'string' && Object.hasOwn(grammar.rules, name);
  const fresh = name => typeof name === 'string' && /^_[A-Za-z_][A-Za-z0-9_]*$/u.test(name)
    && !existing(name) && !(grammar.externals ?? []).some(node => node.name === name);
  if (!Array.isArray(clones) || !clones.length || !clones.every(clone => existing(clone.rule) && fresh(clone.helper))
    || new Set(clones.map(clone => clone.helper)).size !== clones.length || !Array.isArray(fields) || !fields.length) {
    throw new TypeError('field expression contexts need source rules, distinct hidden clones and field selectors');
  }
  const helpers = new Set(clones.map(clone => clone.helper));
  const count = value => Number.isSafeInteger(value) && value > 0;
  const fieldPolicy = policy => policy && typeof policy.field === 'string' && policy.field && existing(policy.symbol)
    && (helpers.has(policy.helper) || existing(policy.helper)) && count(policy.count)
    && (policy.precedence === undefined || Number.isSafeInteger(policy.precedence));
  for (const clone of clones) {
    if (clone.references !== undefined && (!clone.references || typeof clone.references !== 'object' || Array.isArray(clone.references)
      || !Object.entries(clone.references).every(([name, helper]) => existing(name) && helpers.has(helper)))) throw new TypeError('context references need source rules and declared helper identities');
    if (clone.exclude !== undefined && (!clone.exclude || typeof clone.exclude !== 'object' || Array.isArray(clone.exclude)
      || !Object.entries(clone.exclude).every(([name, amount]) => existing(name) && count(amount)))) throw new TypeError('context exclusions need source symbols and exact counts');
    if (clone.precedences !== undefined && (!clone.precedences || !Array.isArray(clone.precedences.values)
      || !clone.precedences.values.length || !clone.precedences.values.every(Number.isSafeInteger)
      || new Set(clone.precedences.values).size !== clone.precedences.values.length || !count(clone.precedences.count))) throw new TypeError('context precedence selections need distinct values and an exact count');
    if (clone.fields !== undefined && (!Array.isArray(clone.fields) || !clone.fields.every(fieldPolicy))) throw new TypeError('context clone fields need exact source selectors');
  }
  if (!fields.every(policy => existing(policy.rule) && fieldPolicy(policy))) throw new TypeError('context fields need exact source selectors');
  const contains = (node, name) => node?.type === 'SYMBOL' && node.name === name
    || Array.isArray(node) && node.some(child => contains(child, name))
    || node && typeof node === 'object' && Object.values(node).some(child => contains(child, name));
  const hidden = new Set([...(grammar.inline ?? []), ...(grammar.supertypes ?? [])].map(node => typeof node === 'string' ? node : node.name));
  const rewriteFields = (source, policies) => {
    const observed = policies.map(() => 0);
    const rewrite = (node, precedence = null) => {
      if (Array.isArray(node)) return node.map(child => rewrite(child, precedence));
      if (!node || typeof node !== 'object') return node;
      if (['PREC', 'PREC_LEFT', 'PREC_RIGHT'].includes(node.type)) precedence = node.value;
      if (node.type === 'FIELD') {
        const selected = policies.map((policy, index) => ({ policy, index })).filter(({ policy }) => policy.field === node.name
          && (policy.precedence === undefined || policy.precedence === precedence)
          && node.content?.type === 'SYMBOL' && node.content.name === policy.symbol);
        if (selected.length > 1) throw new TypeError('context field policies overlap');
        if (selected.length === 1) {
          const { policy, index } = selected[0];
          observed[index] += 1;
          return { ...node, content: { type: 'SYMBOL', name: policy.helper } };
        }
      }
      return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value, precedence)]));
    };
    const rewritten = rewrite(source);
    if (policies.some((policy, index) => policy.count !== observed[index])) throw new TypeError('context field occurrence count differs');
    return rewritten;
  };
  const additions = {};
  for (const clone of clones) {
    let source = structuredClone(grammar.rules[clone.rule]);
    if (clone.precedences) {
      let choice = source;
      while (['PREC', 'PREC_LEFT', 'PREC_RIGHT', 'PREC_DYNAMIC'].includes(choice.type)) choice = choice.content;
      if (choice.type !== 'CHOICE') throw new TypeError('context precedence selection needs a source choice');
      choice.members = choice.members.filter(member => {
        while (member.type === 'PREC_DYNAMIC') member = member.content;
        return ['PREC', 'PREC_LEFT', 'PREC_RIGHT'].includes(member.type) && clone.precedences.values.includes(member.value);
      });
      if (choice.members.length !== clone.precedences.count) throw new TypeError('context precedence selection count differs');
    }
    source = rewriteFields(source, clone.fields ?? []);
    const excluded = new Map(Object.keys(clone.exclude ?? {}).map(name => [name, 0]));
    const project = (node, aliased = false) => {
      if (Array.isArray(node)) return node.map(child => project(child, aliased));
      if (!node || typeof node !== 'object') return node;
      if (node.type === 'SYMBOL' && Object.hasOwn(clone.references ?? {}, node.name)) {
        const reference = { type: 'SYMBOL', name: clone.references[node.name] };
        return aliased || node.name.startsWith('_') || hidden.has(node.name) ? reference
          : { type: 'ALIAS', named: true, value: node.name, content: reference };
      }
      let members = node.members;
      if (node.type === 'CHOICE') {
        members = members.filter(member => {
          const selected = [...excluded.keys()].filter(name => contains(member, name));
          selected.forEach(name => excluded.set(name, excluded.get(name) + 1));
          return selected.length === 0;
        });
        if (!members.length) throw new TypeError('a context projection cannot exclude every source alternative');
      }
      return Object.fromEntries(Object.entries(node).map(([key, value]) => [key,
        project(key === 'members' && node.type === 'CHOICE' ? members : value, aliased || node.type === 'ALIAS' && key === 'content')]));
    };
    additions[clone.helper] = project(source);
    if (Object.entries(clone.exclude ?? {}).some(([name, amount]) => excluded.get(name) !== amount)) throw new TypeError('context exclusion occurrence count differs');
  }
  const replacements = {};
  for (const rule of new Set(fields.map(policy => policy.rule))) replacements[rule] = rewriteFields(grammar.rules[rule], fields.filter(policy => policy.rule === rule));
  Object.assign(grammar.rules, additions, replacements);
}
