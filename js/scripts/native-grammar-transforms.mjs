// Reviewed source-grammar transformations. They generate ordinary grammar
// expressions; neither executor needs a language-specific callback.
export function transformNativeSource(source, transformations = []) {
  const grammar = structuredClone(source);
  for (const decision of transformations) {
    if (decision.family === 'alias-pattern-precedence') {
      const { rules, alias, value } = decision;
      if (!Array.isArray(rules) || !rules.length || rules.some((name) => !Object.hasOwn(grammar.rules, name)) || typeof alias !== 'string' || !alias || !Number.isSafeInteger(value)) throw new TypeError('alias pattern precedence needs existing rules, an alias and integer precedence');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'ALIAS' && node.value === alias && node.content?.type === 'PATTERN') {
          changes += 1;
          return { ...node, content: { type: 'TOKEN', content: { type: 'PREC', value, content: node.content } } };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      for (const name of rules) grammar.rules[name] = rewrite(grammar.rules[name]);
      if (!changes) throw new TypeError('the selected rules contain no matching aliased patterns');
      continue;
    }
    if (decision.family === 'rule-variant-precedence') {
      const { rule, fields, associativity, value, dynamic = 0 } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || !fields || !Object.keys(fields).length || !['left', 'right', 'none'].includes(associativity) || !Number.isSafeInteger(value) || !Number.isSafeInteger(dynamic) || dynamic < 0) throw new TypeError('variant precedence needs an existing rule, field selectors and integer precedence');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (['PREC', 'PREC_LEFT', 'PREC_RIGHT'].includes(node.type) && node.content?.type === 'SEQ' && Object.entries(fields).every(([name, symbol]) => node.content.members.some((member) => member.type === 'FIELD' && member.name === name && member.content.type === 'SYMBOL' && member.content.name === symbol))) {
          changes += 1;
          const production = { ...node, type: associativity === 'none' ? 'PREC' : `PREC_${associativity.toUpperCase()}`, value };
          return dynamic ? { type: 'PREC_DYNAMIC', value: dynamic, content: production } : production;
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('variant precedence must select exactly one production');
      continue;
    }
    if (decision.family === 'context-rule-variants') {
      const { rules, insertInto, prefix, opaqueRules = [] } = decision;
      if (!Array.isArray(rules) || !rules.length || new Set(rules).size !== rules.length || !Array.isArray(insertInto) || !insertInto.length || typeof prefix !== 'string' || !/^_[A-Za-z_]+$/u.test(prefix)) throw new TypeError('context variants need distinct rules, insertion rules and a hidden helper prefix');
      const selected = new Set(rules);
      const silent = new Set([...(grammar.inline ?? []), ...(grammar.supertypes ?? [])]);
      for (const name of [...rules, ...insertInto]) if (!Object.hasOwn(grammar.rules, name)) throw new TypeError(`the context variant rule ${name} is absent`);
      for (const name of rules) if (Object.hasOwn(grammar.rules, prefix + name)) throw new TypeError('a context helper must have a unique name');
      for (const name of opaqueRules) if (!selected.has(name)) throw new TypeError('an opaque context rule must be selected');
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'SYMBOL' && selected.has(node.name)) {
          const reference = { type: 'SYMBOL', name: prefix + node.name };
          return silent.has(node.name) || node.name.startsWith('_') ? reference : { type: 'ALIAS', named: true, value: node.name, content: reference };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      for (const name of rules) grammar.rules[prefix + name] = opaqueRules.includes(name) ? structuredClone(grammar.rules[name]) : rewrite(grammar.rules[name]);
      grammar.inline = [...(grammar.inline ?? []), ...rules.filter((name) => silent.has(name)).map((name) => prefix + name)];
      for (const name of insertInto) grammar.rules[name] = rewrite(grammar.rules[name]);
      continue;
    }
    if (decision.family === 'prefer-optional-literal') {
      const { rule, literal, value } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || typeof literal !== 'string' || !literal || !Number.isSafeInteger(value) || value <= 0) throw new TypeError('an optional literal preference needs an existing rule, literal and positive integer weight');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'CHOICE' && node.members.length === 2 && node.members.some((member) => member.type === 'BLANK') && node.members.some((member) => member.type === 'STRING' && member.value === literal)) {
          changes += 1;
          return { ...node, members: node.members.map((member) => member.type === 'BLANK' ? member : { type: 'PREC_DYNAMIC', value, content: member }) };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('an optional literal preference must identify exactly one existing optional literal');
      continue;
    }
    if (decision.family === 'field-choice-variant') {
      const { rule, helper, field, symbol, alias, insertInto } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || !helper.startsWith('_') || Object.hasOwn(grammar.rules, helper)) throw new TypeError('a field choice variant needs an existing rule and unique hidden helper');
      let changes = 0;
      const select = (node) => {
        if (Array.isArray(node)) return node.map(select);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'FIELD' && node.name === field) {
          if (node.content.type !== 'CHOICE' || !node.content.members.some((item) => item.type === 'SYMBOL' && item.name === symbol)) throw new TypeError('the selected field alternative is absent');
          changes += 1;
          return { ...node, content: { type: 'SYMBOL', name: symbol } };
        }
        return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, select(value)]));
      };
      grammar.rules[helper] = select(grammar.rules[rule]);
      if (changes !== 1 || grammar.rules[insertInto]?.type !== 'CHOICE') throw new TypeError('a field choice variant needs one field and a choice insertion rule');
      grammar.rules[insertInto].members.push({ type: 'ALIAS', named: true, value: alias, content: { type: 'SYMBOL', name: helper } });
      continue;
    }
    if (decision.family === 'ordered-variants') {
      const { rule, variants } = decision;
      let choice = grammar.rules[rule];
      while (choice && ['PREC', 'PREC_LEFT', 'PREC_RIGHT'].includes(choice.type)) choice = choice.content;
      if (choice?.type !== 'CHOICE' || !Array.isArray(variants) || variants.length < 2) throw new TypeError('ordered variants need a choice and at least two alternatives');
      const selector = (node) => {
        while (['PREC', 'PREC_LEFT', 'PREC_RIGHT'].includes(node.type)) node = node.content;
        return node.type === 'SYMBOL' ? node.name : node.type === 'ALIAS' ? `alias:${node.value}` : null;
      };
      const indices = variants.map((name) => choice.members.findIndex((node) => selector(node) === name));
      if (indices.some((index) => index < 0) || new Set(indices).size !== indices.length) throw new TypeError('ordered variants must identify distinct existing alternatives');
      const ordered = { type: 'NATIVE_ORDERED_CHOICE', members: indices.map((index) => choice.members[index]) };
      const first = Math.min(...indices);
      choice.members = choice.members.flatMap((node, index) => index === first ? [ordered] : indices.includes(index) ? [] : [node]);
      continue;
    }
    if (decision.family === 'rule-precedence') {
      const { rule, associativity, value } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || !['left', 'right', 'none'].includes(associativity) || !Number.isSafeInteger(value)) throw new TypeError('a rule precedence needs an existing rule, associativity and integer value');
      const original = grammar.rules[rule];
      grammar.rules[rule] = { type: associativity === 'none' ? 'PREC' : `PREC_${associativity.toUpperCase()}`, value, content: ['PREC', 'PREC_LEFT', 'PREC_RIGHT'].includes(original.type) ? original.content : original };
      continue;
    }
    if (decision.family !== 'nullable-pattern-extras') throw new TypeError(`unknown native grammar transformation ${decision.family}`);
    const { rule, pattern, extraAlternative, helper, whitespace } = decision;
    if (!/^[A-Za-z_][A-Za-z0-9_]*$/u.test(helper) || !helper.startsWith('_') || Object.hasOwn(grammar.rules, helper)) throw new TypeError('a nullable-pattern extra needs a unique hidden helper');
    const choice = grammar.rules[rule];
    if (choice?.type !== 'CHOICE' || !Number.isInteger(extraAlternative) || !choice.members[extraAlternative]) throw new TypeError('the nullable-pattern extra alternative is absent');
    grammar.rules[helper] = structuredClone(choice.members[extraAlternative]);
    let replaced = 0;
    const rewrite = (node) => {
      if (Array.isArray(node)) return node.map(rewrite);
      if (!node || typeof node !== 'object') return node;
      const alias = node.type === 'FIELD' ? node.content : node;
      if (alias?.type === 'ALIAS' && alias.content?.type === 'PATTERN' && alias.content.value === pattern) {
        replaced += 1;
        const token = { ...alias, content: { type: 'IMMEDIATE_TOKEN', content: alias.content } };
        return { type: 'SEQ', members: [
          { type: 'REPEAT', content: { type: 'SEQ', members: [
            { type: 'IMMEDIATE_TOKEN', content: { type: 'PATTERN', value: whitespace } },
            { type: 'ALIAS', named: true, value: rule, content: { type: 'SYMBOL', name: helper } },
          ] } },
          node.type === 'FIELD' ? { ...node, content: token } : token,
        ] };
      }
      return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
    };
    grammar.rules[rule] = rewrite(choice);
    if (replaced !== 1) throw new TypeError('the nullable-pattern extra must identify exactly one aliased pattern');
  }
  return grammar;
}
