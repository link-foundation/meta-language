// Reviewed source-grammar transformations. They generate ordinary grammar
// expressions; neither executor needs a language-specific callback.
export function transformNativeSource(source, transformations = []) {
  const grammar = structuredClone(source);
  for (const decision of transformations) {
    if (decision.family === 'external-literal-token') {
      const { literal, token, count } = decision;
      if (typeof literal !== 'string' || !literal || typeof token !== 'string' || !/^_[A-Za-z_]+$/u.test(token) || !Number.isSafeInteger(count) || count <= 0
        || Object.hasOwn(grammar.rules, token) || (grammar.externals ?? []).some((external) => external.name === token)) throw new TypeError('external literal tokens need a literal, a unique hidden token and a positive production count');
      let declarations = 0, changes = 0;
      grammar.externals = (grammar.externals ?? []).map((external) => {
        if (external.type !== 'STRING' || external.value !== literal) return external;
        declarations += 1;
        return { type: 'SYMBOL', name: token };
      });
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'STRING' && node.value === literal) {
          changes += 1;
          return { type: 'ALIAS', named: false, value: literal, content: { type: 'SYMBOL', name: token } };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      grammar.rules = rewrite(grammar.rules);
      if (declarations !== 1 || changes !== count) throw new TypeError('external literal tokens must select exactly the recorded external and productions');
      continue;
    }
    if (decision.family === 'contextual-newline-extras') {
      const { pattern, token } = decision;
      if (typeof pattern !== 'string' || !pattern || typeof token !== 'string' || !/^_[A-Za-z_]+$/u.test(token)
        || Object.hasOwn(grammar.rules, token) || (grammar.externals ?? []).some((external) => external.name === token)) throw new TypeError('contextual newline extras need a source pattern and a unique hidden scanner token');
      let changes = 0;
      grammar.extras = (grammar.extras ?? []).map((extra) => {
        if (extra.type !== 'PATTERN' || extra.value !== pattern) return extra;
        changes += 1;
        return { type: 'NATIVE_PREFIX_EXCLUSION', content: extra, prefixes: { type: 'STRING', value: '\n' } };
      });
      if (changes !== 1) throw new TypeError('contextual newline extras must select exactly one source pattern');
      grammar.externals = [...(grammar.externals ?? []), { type: 'SYMBOL', name: token }];
      grammar.extras.push({ type: 'SYMBOL', name: token });
      continue;
    }
    if (decision.family === 'rule-prefix-exclusion') {
      const { rule, pattern, continuationPattern = null } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || typeof pattern !== 'string' || !pattern) throw new TypeError('rule prefix exclusions need an existing rule and nonempty pattern');
      if (continuationPattern !== null && (typeof continuationPattern !== 'string' || !continuationPattern)) throw new TypeError('prefix continuations need a nonempty pattern');
      const prefix = { type: 'PATTERN', value: pattern };
      grammar.rules[rule] = { type: 'NATIVE_PREFIX_EXCLUSION', content: grammar.rules[rule], prefixes: continuationPattern === null ? prefix : { type: 'NATIVE_WORD_BOUNDARY', content: prefix, word: { type: 'PATTERN', value: continuationPattern } } };
      continue;
    }
    if (['pattern-prefix-exclusion', 'literal-prefix-exclusion'].includes(decision.family)) {
      const { rule, pattern, excludedPattern } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || ![pattern, excludedPattern].every((value) => typeof value === 'string' && value)) throw new TypeError('pattern exclusions need a rule and nonempty patterns');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === (decision.family === 'pattern-prefix-exclusion' ? 'PATTERN' : 'STRING') && node.value === pattern) {
          changes += 1;
          return { type: 'NATIVE_PREFIX_EXCLUSION', content: node, prefixes: { type: 'PATTERN', value: excludedPattern } };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('pattern exclusions must select exactly one source pattern');
      continue;
    }
    if (decision.family === 'literal-continuation-exclusion') {
      const { rules, literal, continuationPattern } = decision;
      if (!Array.isArray(rules) || !rules.length || rules.some((name) => !Object.hasOwn(grammar.rules, name)) || ![literal, continuationPattern].every((value) => typeof value === 'string' && value)) throw new TypeError('literal boundaries need existing rules and nonempty literal and continuation pattern');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'STRING' && node.value === literal) { changes += 1; return { type: 'NATIVE_LITERAL_BOUNDARY', content: node, continuationPattern }; }
        return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
      };
      for (const rule of rules) grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (!changes) throw new TypeError('literal boundaries found no selected literal');
      continue;
    }
    if (['field-dynamic-precedence', 'field-present-precedence', 'field-absent-boundary'].includes(decision.family)) {
      const { rule, field, value, boundary, symbol = null } = decision;
      const absent = decision.family === 'field-absent-boundary';
      const present = decision.family === 'field-present-precedence';
      if (!Object.hasOwn(grammar.rules, rule) || typeof field !== 'string' || !field || (absent ? typeof boundary !== 'string' || !boundary : !Number.isSafeInteger(value) || value <= 0)) throw new TypeError('field preferences need an existing rule, nonempty field and boundary or positive preference');
      if (symbol !== null && !Object.hasOwn(grammar.rules, symbol)) throw new TypeError('field symbol preferences need an existing symbol');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'FIELD' && node.name === field && (symbol === null || node.content.type === 'SYMBOL' && node.content.name === symbol)) {
          changes += 1;
          if ((absent || present) && (node.content.type !== 'CHOICE' || !node.content.members.some((member) => member.type === 'BLANK'))) throw new TypeError('optional field preferences need an optional field');
          const content = absent ? { type: 'NATIVE_COMPLETE_CONTEXT_VARIANT', preferred: { type: 'PREC_DYNAMIC', value: 1, content: { type: 'BLANK' } }, boundary, content: node.content }
            : present ? { ...node.content, members: node.content.members.map((member) => member.type === 'BLANK' ? member : { type: 'PREC_DYNAMIC', value, content: member }) }
              : { type: 'PREC_DYNAMIC', value, content: node.content };
          return { ...node, content };
        }
        return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('field preferences must select exactly one field');
      continue;
    }
    if (decision.family === 'rule-word-boundary') {
      const { rules, wordRule, continuationPattern = null } = decision;
      if (!Array.isArray(rules) || !rules.length || new Set(rules).size !== rules.length || rules.some((name) => !Object.hasOwn(grammar.rules, name)) || grammar.rules[wordRule]?.type !== 'PATTERN') throw new TypeError('word boundaries need distinct existing rules and a word pattern');
      if (continuationPattern !== null && (typeof continuationPattern !== 'string' || !continuationPattern)) throw new TypeError('word continuations need a nonempty pattern');
      for (const rule of rules) {
        if (grammar.rules[rule].type !== 'PATTERN') throw new TypeError('word boundaries apply to pattern tokens');
        grammar.rules[rule] = { type: 'NATIVE_WORD_BOUNDARY', content: grammar.rules[rule], word: continuationPattern === null ? grammar.rules[wordRule] : { type: 'PATTERN', value: continuationPattern } };
      }
      continue;
    }
    if (decision.family === 'symbol-prefix-exclusion') {
      const { rule, symbol, prefixRule } = decision;
      if (![rule, symbol, prefixRule].every((name) => Object.hasOwn(grammar.rules, name))) throw new TypeError('prefix exclusions need existing target, symbol and prefix rules');
      const prefixes = [];
      const first = (node) => {
        if (node.type === 'STRING' && node.value) prefixes.push(node);
        else if (node.type === 'SEQ') first(node.members[0]);
        else if (node.type === 'CHOICE') node.members.forEach(first);
        else if (node.content) first(node.content);
      };
      first(grammar.rules[prefixRule]);
      if (!prefixes.length) throw new TypeError('a prefix rule needs initial literal alternatives');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'SYMBOL' && node.name === symbol) { changes += 1; return { type: 'NATIVE_PREFIX_EXCLUSION', content: node, prefixes: { type: 'CHOICE', members: prefixes } }; }
        return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (!changes) throw new TypeError('the target rule contains no selected symbol');
      continue;
    }
    if (decision.family === 'field-ordered-variants') {
      const { rule, field, variants } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || typeof field !== 'string' || !field || !Array.isArray(variants) || variants.length < 2 || new Set(variants).size !== variants.length) throw new TypeError('field ordering needs an existing rule, field and distinct alternatives');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'FIELD' && node.name === field && node.content.type === 'CHOICE') {
          const members = node.content.members;
          const selected = variants.map((name) => members.find((member) => member.type === 'SYMBOL' && member.name === name));
          if (selected.some((member) => !member) || selected.length !== members.length) throw new TypeError('field ordering must cover each existing alternative exactly once');
          changes += 1;
          return { ...node, content: { type: 'NATIVE_ORDERED_CHOICE', members: selected } };
        }
        return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('field ordering must select exactly one choice');
      continue;
    }
    if (decision.family === 'field-literal-alternative') {
      const { rule, field, literal, alias } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || ![field, literal, alias].every((value) => typeof value === 'string' && value)) throw new TypeError('field literal alternatives need an existing rule and nonempty field, literal and alias');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'FIELD' && node.name === field) {
          changes += 1;
          return { ...node, content: { type: 'CHOICE', members: [{ type: 'ALIAS', named: true, value: alias, content: { type: 'STRING', value: literal } }, node.content] } };
        }
        return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('field literal alternatives must select exactly one field');
      continue;
    }
    if (decision.family === 'pattern-trivia-run') {
      const { rule, pattern } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || typeof pattern !== 'string' || !pattern) throw new TypeError('pattern runs need an existing rule and nonempty pattern');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'PATTERN' && node.value === pattern) { changes += 1; return { type: 'TOKEN', content: { type: 'REPEAT1', content: node } }; }
        return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('pattern runs must select exactly one pattern');
      continue;
    }
    if (['complete-context-variant', 'keyword-context-variant', 'pattern-context-variant', 'complete-alternative-preference'].includes(decision.family)) {
      const { rule, symbol = null, field = null, boundary, excludedKeywords = [], includedKeywords = [], wordRule = null } = decision;
      const patternOnly = decision.family === 'pattern-context-variant';
      const keywordOnly = ['keyword-context-variant', 'complete-alternative-preference'].includes(decision.family) || patternOnly;
      if (patternOnly && (typeof decision.pattern !== 'string' || !decision.pattern || includedKeywords.length || excludedKeywords.length)) throw new TypeError('a pattern context variant needs a nonempty pattern and no keyword guard');
      if (decision.family === 'keyword-context-variant' && !includedKeywords.length && !excludedKeywords.length) throw new TypeError('a keyword context variant needs a keyword guard');
      const declaredSymbol = symbol === null || Object.hasOwn(grammar.rules, symbol) || (grammar.externals ?? []).some((external) => external.type === 'SYMBOL' && external.name === symbol);
      if (!Object.hasOwn(grammar.rules, rule) || (symbol === null) === (field === null) || !declaredSymbol || (field !== null && (typeof field !== 'string' || !field)) || (!keywordOnly && boundary !== null && (typeof boundary !== 'string' || !boundary))) throw new TypeError('a complete context variant needs an existing rule, a symbol or field selector and a literal or input boundary');
      let choice = grammar.rules[rule];
      while (['PREC', 'PREC_LEFT', 'PREC_RIGHT', 'NATIVE_COMPLETE_CONTEXT_VARIANT', 'NATIVE_KEYWORD_CONTEXT_VARIANT'].includes(choice.type)) choice = choice.content;
      if (choice.type !== 'CHOICE') throw new TypeError('a complete context variant needs a choice');
      const alternatives = (node) => node.type === 'CHOICE' ? node.members.flatMap(alternatives) : [node];
      const selected = alternatives(choice).filter((member) => {
        while (['PREC', 'PREC_LEFT', 'PREC_RIGHT', 'PREC_DYNAMIC'].includes(member.type)) member = member.content;
        const first = decision.family === 'complete-alternative-preference' && member.type === 'SEQ' ? member.members[0] : member;
        return (symbol !== null && first.type === 'SYMBOL' && first.name === symbol) || (field !== null && member.type === 'FIELD' && member.name === field);
      });
      if (selected.length !== 1) throw new TypeError('a complete context variant must select exactly one existing alternative');
      let preferred = selected[0];
      if (patternOnly) preferred = { type: 'NATIVE_PATTERN_LOOKAHEAD', content: preferred, pattern: decision.pattern };
      if (excludedKeywords.length) {
        if (!Array.isArray(excludedKeywords) || excludedKeywords.some((word) => typeof word !== 'string' || !word) || !Object.hasOwn(grammar.rules, wordRule)) throw new TypeError('context keyword exclusions need words and an existing word rule');
        preferred = { type: 'NATIVE_KEYWORD_EXCLUSION', content: preferred, keywords: { type: 'CHOICE', members: excludedKeywords.map((value) => ({ type: 'STRING', value })) }, word: grammar.rules[wordRule] };
      }
      if (includedKeywords.length) {
        if (!Array.isArray(includedKeywords) || excludedKeywords.length || includedKeywords.some((word) => typeof word !== 'string' || !word) || !Object.hasOwn(grammar.rules, wordRule)) throw new TypeError('context keyword requirements need words and an existing word rule');
        preferred = { type: 'NATIVE_KEYWORD_REQUIREMENT', content: preferred, keywords: { type: 'CHOICE', members: includedKeywords.map((value) => ({ type: 'STRING', value })) }, word: grammar.rules[wordRule] };
      }
      grammar.rules[rule] = { type: keywordOnly ? 'NATIVE_KEYWORD_CONTEXT_VARIANT' : 'NATIVE_COMPLETE_CONTEXT_VARIANT', preferred, boundary, content: grammar.rules[rule] };
      continue;
    }
    if (decision.family === 'optional-suffix-context') {
      const { rule, prefix, suffix, helper, requiredSymbol, value = 1 } = decision;
      if (![rule, prefix, suffix, requiredSymbol].every((name) => Object.hasOwn(grammar.rules, name)) || typeof helper !== 'string' || !/^_[A-Za-z_]+$/u.test(helper) || Object.hasOwn(grammar.rules, helper) || !Number.isSafeInteger(value) || value <= 0) throw new TypeError('an optional suffix context needs existing rules, a unique hidden helper and positive preference');
      let requirements = 0;
      const requireSymbol = (node) => {
        if (Array.isArray(node)) return node.map(requireSymbol);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'CHOICE' && node.members.length === 2 && node.members.some((member) => member.type === 'BLANK') && node.members.some((member) => member.type === 'SYMBOL' && member.name === requiredSymbol)) {
          requirements += 1;
          return node.members.find((member) => member.type !== 'BLANK');
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, requireSymbol(child)]));
      };
      grammar.rules[helper] = requireSymbol(structuredClone(grammar.rules[prefix]));
      if (requirements !== 1) throw new TypeError('an optional suffix context must identify exactly one optional prefix symbol');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'SEQ' && node.members.length === 2 && node.members[0].type === 'SYMBOL' && node.members[0].name === prefix && node.members[1].type === 'CHOICE' && node.members[1].members.length === 2 && node.members[1].members.some((member) => member.type === 'BLANK') && node.members[1].members.some((member) => member.type === 'SYMBOL' && member.name === suffix)) {
          changes += 1;
          return { type: 'NATIVE_OPTIONAL_SUFFIX_CONTEXT', content: node, helper, value };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('an optional suffix context must select one existing prefix and optional suffix sequence');
      continue;
    }
    if (decision.family === 'symbol-dynamic-precedence') {
      const { rules, symbol, value } = decision;
      const declaredSymbol = Object.hasOwn(grammar.rules, symbol) || (grammar.externals ?? []).some((external) => external.type === 'SYMBOL' && external.name === symbol);
      if (!Array.isArray(rules) || !rules.length || rules.some((name) => !Object.hasOwn(grammar.rules, name)) || !declaredSymbol || !Number.isSafeInteger(value) || value <= 0) throw new TypeError('a symbol preference needs existing rules, symbol and positive integer weight');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'SYMBOL' && node.name === symbol) {
          changes += 1;
          return { type: 'PREC_DYNAMIC', value, content: node };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      for (const rule of rules) grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (!changes) throw new TypeError('the selected rules contain no matching symbols');
      continue;
    }
    if (decision.family === 'literal-end-boundary') {
      const { rule, literal, alias = null } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || typeof literal !== 'string' || !literal || (alias !== null && typeof alias !== 'string')) throw new TypeError('an end boundary needs an existing rule, nonempty sentinel literal and optional alias');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'STRING' && node.value === literal) {
          changes += 1;
          const content = { type: 'NATIVE_END_BOUNDARY', content: node };
          return alias === null ? content : { type: 'ALIAS', named: false, value: alias, content };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('an end boundary must identify exactly one sentinel literal');
      continue;
    }
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
    if (decision.family === 'alias-choice-branches') {
      const { rule, alias, helpers, suffix } = decision;
      const source = grammar.rules[rule];
      if (source?.type !== 'ALIAS' || !source.named || source.value !== alias || source.content?.type !== 'CHOICE'
        || !Array.isArray(helpers) || helpers.length !== source.content.members.length || new Set(helpers).size !== helpers.length
        || helpers.some((helper) => typeof helper !== 'string' || !/^_[A-Za-z_]+$/u.test(helper) || Object.hasOwn(grammar.rules, helper))
        || typeof suffix !== 'string' || !suffix) throw new TypeError('alias branches need a named choice, unique hidden helpers and a suffix literal');
      const members = source.content.members.map((member, index) => {
        const last = member.type === 'SEQ' ? member.members.at(-1) : null;
        const optionalSuffix = last?.type === 'CHOICE' && last.members.length === 2
          && last.members.some((item) => item.type === 'BLANK')
          && last.members.some((item) => item.type === 'STRING' && item.value === suffix);
        const prefix = optionalSuffix ? { type: 'SEQ', members: member.members.slice(0, -1) } : member;
        grammar.rules[helpers[index]] = prefix;
        const body = { ...source, content: { type: 'SYMBOL', name: helpers[index] } };
        return optionalSuffix ? { type: 'SEQ', members: [body, {
          type: 'CHOICE', members: [{ ...source, content: { type: 'STRING', value: suffix } }, { type: 'BLANK' }],
        }] } : body;
      });
      grammar.rules[rule] = { type: 'CHOICE', members };
      continue;
    }
    if (decision.family === 'alias-choice-rule') {
      const { rule, alias, helper } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || typeof alias !== 'string' || !alias || typeof helper !== 'string' || !/^_[A-Za-z_]+$/u.test(helper) || Object.hasOwn(grammar.rules, helper)) throw new TypeError('alias choice rules need an existing rule, alias and unique hidden helper');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        const body = node.type === 'ALIAS' && node.content?.type === 'SYMBOL'
          ? grammar.rules[node.content.name] : node.content;
        if (node.type === 'ALIAS' && node.named && node.value === alias && body?.type === 'CHOICE') {
          changes += 1;
          grammar.rules[helper] = structuredClone(body);
          return { ...node, content: { type: 'SYMBOL', name: helper } };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('alias choice rules must select exactly one named choice');
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
        if (node.type === 'CHOICE' && node.members.length === 2 && node.members.some((member) => member.type === 'BLANK') && node.members.some((member) => {
          while (['TOKEN', 'IMMEDIATE_TOKEN', 'PREC'].includes(member.type)) member = member.content;
          return member.type === 'STRING' && member.value === literal;
        })) {
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
      const { rule, variants, lookaheadPatterns = {} } = decision;
      let choice = grammar.rules[rule];
      while (choice && ['PREC', 'PREC_LEFT', 'PREC_RIGHT'].includes(choice.type)) choice = choice.content;
      if (choice?.type !== 'CHOICE' || !Array.isArray(variants) || variants.length < 2) throw new TypeError('ordered variants need a choice and at least two alternatives');
      const selector = (node) => {
        while (['PREC', 'PREC_LEFT', 'PREC_RIGHT'].includes(node.type)) node = node.content;
        return node.type === 'SYMBOL' ? node.name : node.type === 'ALIAS' ? `alias:${node.value}` : null;
      };
      const indices = variants.map((name) => choice.members.findIndex((node) => selector(node) === name));
      if (indices.some((index) => index < 0) || new Set(indices).size !== indices.length) throw new TypeError('ordered variants must identify distinct existing alternatives');
      for (const [name, pattern] of Object.entries(lookaheadPatterns)) if (!variants.includes(name) || typeof pattern !== 'string' || !pattern) throw new TypeError('ordered lookaheads need a selected variant and a nonempty pattern');
      const ordered = { type: 'NATIVE_ORDERED_CHOICE', members: indices.map((index, position) => {
        const name = variants[position];
        let content = choice.members[index];
        if (Object.hasOwn(lookaheadPatterns, name)) content = { type: 'NATIVE_PATTERN_LOOKAHEAD', content, pattern: lookaheadPatterns[name] };
        return content;
      }) };
      const first = Math.min(...indices);
      choice.members = choice.members.flatMap((node, index) => index === first ? [ordered] : indices.includes(index) ? [] : [node]);
      continue;
    }
    if (decision.family === 'symbol-keyword-exclusion') {
      const { rule, symbol, keywordRules = [], keywords = [], keywordPatterns = false, wordRule } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || !Object.hasOwn(grammar.rules, symbol) || !Object.hasOwn(grammar.rules, wordRule) || typeof keywordPatterns !== 'boolean' || !Array.isArray(keywordRules) || !Array.isArray(keywords) || (!keywordRules.length && !keywords.length) || keywords.some((word) => typeof word !== 'string' || !word) || keywordRules.some((name) => !Object.hasOwn(grammar.rules, name))) throw new TypeError('keyword exclusions need existing target, word and keyword rules');
      const patterns = keywords.map((value) => ({ type: 'STRING', value }));
      const first = (node) => {
        if (keywordPatterns && ['PATTERN', 'STRING'].includes(node.type)) { patterns.push(node); return; }
        if (node.type === 'ALIAS' && !node.named) {
          let token = node.content;
          while (['PREC', 'TOKEN', 'IMMEDIATE_TOKEN'].includes(token.type)) token = token.content;
          if (['PATTERN', 'STRING'].includes(token.type)) { patterns.push(token); return; }
        }
        if (node.type === 'SEQ') first(node.members[0]);
        else if (node.type === 'CHOICE') node.members.forEach(first);
        else if (node.content) first(node.content);
      };
      keywordRules.forEach((name) => first(grammar.rules[name]));
      if (!patterns.length) throw new TypeError('the selected rules have no initial anonymous keyword tokens');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (node.type === 'SYMBOL' && node.name === symbol) {
          changes += 1;
          return { type: 'NATIVE_KEYWORD_EXCLUSION', content: node, keywords: { type: 'CHOICE', members: patterns }, word: grammar.rules[wordRule] };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      grammar.rules[rule] = rewrite(grammar.rules[rule]);
      if (changes !== 1) throw new TypeError('a keyword exclusion must select one contextual symbol');
      continue;
    }
    if (decision.family === 'lift-token-aliases') {
      const { rules } = decision;
      if (!Array.isArray(rules) || !rules.length || rules.some((name) => !Object.hasOwn(grammar.rules, name))) throw new TypeError('token alias lifting needs existing rules');
      let changes = 0;
      const rewrite = (node) => {
        if (Array.isArray(node)) return node.map(rewrite);
        if (!node || typeof node !== 'object') return node;
        if (['TOKEN', 'IMMEDIATE_TOKEN'].includes(node.type) && node.content?.type === 'ALIAS') {
          changes += 1;
          return { ...node.content, content: { ...node, content: rewrite(node.content.content) } };
        }
        return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
      };
      for (const name of rules) grammar.rules[name] = rewrite(grammar.rules[name]);
      if (!changes) throw new TypeError('the selected rules contain no token aliases');
      continue;
    }
    if (decision.family === 'rule-dynamic-precedence') {
      const { rule, value } = decision;
      if (!Object.hasOwn(grammar.rules, rule) || !Number.isSafeInteger(value) || value <= 0) throw new TypeError('a dynamic rule preference needs an existing rule and a positive integer');
      grammar.rules[rule] = { type: 'PREC_DYNAMIC', value, content: grammar.rules[rule] };
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
