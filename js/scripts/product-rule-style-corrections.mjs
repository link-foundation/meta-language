// Reviewed corrections for the published 285c8b13 native lint log. These are
// source styles, not invented machine-applicable compiler suggestions.
import { tokenize } from '../src/translation/lexer.js';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

/** Format bounded generated source through a temporary file and always remove it. */
export function formatProductRuleSource(source) {
  const directory = mkdtempSync(join(tmpdir(), 'meta-language-product-style-'));
  try {
    const file = join(directory, 'rules.rs');
    writeFileSync(file, source);
    const result = spawnSync('rustfmt', ['--edition', '2024', '--config', 'skip_children=true', '--emit', 'files', file], {
      encoding: 'utf8', maxBuffer: 4 * 1024 * 1024, timeout: 20000,
    });
    if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
    return readFileSync(file, 'utf8');
  } finally { rmSync(directory, { recursive: true, force: true }); }
}

function syntax(source) {
  const tokens = tokenize(source, 'Rust').tokens;
  const matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    const value = tokens[index].value;
    if (['(', '[', '{'].includes(value)) stack.push(index);
    if ([')', ']', '}'].includes(value)) {
      const open = stack.pop();
      if (open === undefined || ({ ')': '(', ']': '[', '}': '{' })[value] !== tokens[open].value) throw new Error('product style input has unbalanced delimiters');
      matching.set(open, index); matching.set(index, open);
    }
  }
  return { tokens, matching };
}

function definitions(source) {
  const { tokens, matching } = syntax(source), result = [];
  for (let index = 0; index < tokens.length - 2; index += 1) {
    if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'fn') continue;
    const name = tokens[index + 1].value;
    let open = index + 2;
    while (open < tokens.length && tokens[open].value !== '(') open += 1;
    let body = matching.get(open) + 1;
    while (body < tokens.length && tokens[body].value !== '{') body += 1;
    if (!matching.has(body)) throw new Error(`product style cannot read ${name}`);
    const start = tokens[index - 1]?.value === 'pub' ? tokens[index - 1].start : tokens[index].start;
    result.push({ name, start, end: tokens[matching.get(body)].end });
    index = matching.get(body);
  }
  return result;
}

function conditional(tokens, matching, start) {
  let body = start + 1;
  while (body < tokens.length) {
    if (tokens[body].value === 'match') {
      body += 1;
      while (tokens[body].value !== '{') {
        if (['(', '['].includes(tokens[body].value)) body = matching.get(body) + 1;
        else body += 1;
      }
      body = matching.get(body) + 1;
      continue;
    }
    if (tokens[body].kind === 'punct' && tokens[body].value === '{') break;
    if (tokens[body].kind === 'punct' && ['(', '['].includes(tokens[body].value)) body = matching.get(body) + 1;
    else body += 1;
  }
  const close = matching.get(body);
  if (close === undefined) throw new Error('product style cannot read an if body');
  let alternate = null, end = close;
  if (tokens[close + 1]?.value === 'else') {
    const next = close + 2;
    if (tokens[next].value === '{') { alternate = { open: next, close: matching.get(next) }; end = alternate.close; }
    else if (tokens[next].value === 'if') { alternate = conditional(tokens, matching, next); end = alternate.end; }
  }
  return { start, open: body, close, alternate, end };
}

function continues(tokens, matching, open, close) {
  let last = open + 1;
  for (let index = open + 1; index < close;) {
    last = index;
    if (tokens[index].value === 'if') index = conditional(tokens, matching, index).end + 1;
    else if (tokens[index].kind === 'punct' && ['(', '[', '{'].includes(tokens[index].value)) index = matching.get(index) + 1;
    else index += 1;
    if (tokens[index]?.value === ';' && index < close) index += 1;
  }
  if (tokens[last]?.value === 'continue') return true;
  if (tokens[last]?.value === '{') return continues(tokens, matching, last, matching.get(last));
  if (tokens[last]?.value !== 'if') return false;
  const node = conditional(tokens, matching, last);
  const branchesContinue = node => continues(tokens, matching, node.open, node.close) && Boolean(node.alternate)
    && (node.alternate.start !== undefined ? branchesContinue(node.alternate)
      : continues(tokens, matching, node.alternate.open, node.alternate.close));
  return branchesContinue(node);
}

function simplifyBranches(source) {
  for (let round = 0; round < 100; round += 1) {
    const { tokens, matching } = syntax(source);
    let changed = false;
    for (let index = tokens.length - 1; index >= 0; index -= 1) {
      if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'if') continue;
      const node = conditional(tokens, matching, index);
      if (node.alternate?.start === undefined && node.alternate
        && continues(tokens, matching, node.open, node.close)) {
        const prefix = tokens[index - 1]?.value === 'return' ? tokens[index - 1].start : tokens[index].start;
        const statement = source.slice(tokens[index].start, tokens[node.close].end);
        const fallback = source.slice(tokens[node.alternate.open].end, tokens[node.alternate.close].start);
        const isReturn = prefix !== tokens[index].start;
        const replacement = isReturn ? `${statement}\nreturn {${fallback}}` : `${statement}\n{${fallback}}`;
        source = source.slice(0, prefix) + replacement + source.slice(tokens[node.end].end);
        changed = true; break;
      }
      if (node.alternate) {
        const inner = node.alternate.open + 1;
        if (node.alternate.start === undefined && tokens[inner]?.value === 'if') {
          const child = conditional(tokens, matching, inner);
          if (child.end + 1 === node.alternate.close) {
            source = source.slice(0, tokens[node.alternate.open].start)
              + source.slice(tokens[inner].start, tokens[child.end].end)
              + source.slice(tokens[node.alternate.close].end);
            changed = true; break;
          }
        }
        const condition = source.slice(tokens[index].end, tokens[node.open].start).trim();
        if (condition.startsWith('!')) {
          const positive = condition.slice(1).trim();
          const then = source.slice(tokens[node.open].start, tokens[node.close].end);
          const otherwise = node.alternate.start === undefined
            ? source.slice(tokens[node.alternate.open].start, tokens[node.alternate.close].end)
            : `{${source.slice(tokens[node.alternate.start].start, tokens[node.alternate.end].end)}}`;
          source = source.slice(0, tokens[index].start) + `if ${positive} ${otherwise} else ${then}` + source.slice(tokens[node.end].end);
          changed = true; break;
        }
        if (/^\(?kind != "tuple"\)?$/u.test(condition)) {
          const then = source.slice(tokens[node.open].start, tokens[node.close].end);
          const otherwise = node.alternate.start === undefined
            ? source.slice(tokens[node.alternate.open].start, tokens[node.alternate.close].end)
            : `{${source.slice(tokens[node.alternate.start].start, tokens[node.alternate.end].end)}}`;
          source = source.slice(0, tokens[index].start) + `if kind == "tuple" ${otherwise} else ${then}` + source.slice(tokens[node.end].end);
          changed = true; break;
        }
      }
    }
    if (!changed) return source;
  }
  throw new Error('product source branch styles did not converge');
}

function borrowed(source, names) {
  for (const [name, type] of Object.entries(names)) source = source.replace(new RegExp(`\\b(?:mut )?${name}: (?:String|Vec<[^>]+>)`, 'u'), `${name}: ${type}`);
  return source;
}

function arrayModule(source) {
  const { tokens, matching } = syntax(source);
  const start = tokens.findIndex((token, index) => token.value === 'pub' && tokens[index + 1]?.value === 'mod' && tokens[index + 2]?.value === 'ml_array');
  if (start < 0) throw new Error('product style lost the existing array module');
  return source.slice(tokens[start].start, tokens[matching.get(start + 3)].end);
}

function normalizeArrayModule(source) {
  const normalized = source.replace(/match index\.and_then\(\|index\| values\.get\(index\)\) \{\s*Some\(value\) => value\.clone\(\),\s*None => panic!\("array index out of range"\),\s*\}/u,
    'index.and_then(|index| values.get(index)).map_or_else(|| panic!("array index out of range"), Clone::clone)')
    .replace('const fn integer', 'fn integer').replace(/#\[must_use\]\s*/gu, '');
  return JSON.stringify(tokenize(normalized, 'Rust').tokens.filter(token => token.kind !== 'eof')
    .map(token => [token.kind, token.kind === 'number' ? normalized.slice(token.start, token.end).replaceAll('_', '') : token.value]));
}

function simplifyReturns(source) {
  for (let round = 0; round < 100; round += 1) {
    const { tokens, matching } = syntax(source);
    let changed = false;
    for (let close = tokens.length - 1; close >= 0; close -= 1) {
      if (tokens[close].kind !== 'punct' || tokens[close].value !== '}') continue;
      const tail = close - 1;
      if (tokens[tail]?.kind !== 'identifier' || tokens[tail - 1]?.value !== ';') continue;
      let begin = tail - 2;
      while (begin > matching.get(close)) {
        if (tokens[begin].kind === 'punct' && [')', ']', '}'].includes(tokens[begin].value)) begin = matching.get(begin) - 1;
        else if (tokens[begin].value === ';') { begin += 1; break; }
        else begin -= 1;
      }
      if (begin === matching.get(close)) begin += 1;
      if (tokens[begin]?.value !== 'let' || tokens[begin + 1]?.value !== tokens[tail].value || tokens[begin + 2]?.value !== '=') continue;
      const expression = source.slice(tokens[begin + 3].start, tokens[tail - 1].start);
      source = source.slice(0, tokens[begin].start) + expression + source.slice(tokens[tail].end);
      changed = true; break;
    }
    if (!changed) return source;
  }
  throw new Error('product return styles did not converge');
}

/** Build exact function-scoped reviewed styles, preserving the provider ABI. */
export function readProductRuleStyleCorrections(source, family, arrayProvider) {
  const actions = [], originalArray = arrayModule(source);
  if (normalizeArrayModule(originalArray) !== normalizeArrayModule(arrayModule(arrayProvider))) throw new Error('shared product array provider differs in operation, signature or body');
  actions.push({ op: 'replace', field: 'source', from: originalArray, to: 'pub use crate::translation::frontend_rules::ml_array;' });
  source = source.replace(originalArray, actions[0].to);
  for (const definition of definitions(source).reverse()) {
    let from = source.slice(definition.start, definition.end), to = from;
    if (family === 'emission') {
      if (definition.name === 'join_product_items') {
        to = borrowed(to, { items: '&[String]' }).replaceAll('items.clone()', 'items').replaceAll('text.clone()', 'text').replaceAll('text_2.clone()', 'text_2');
      } else if (definition.name === 'render_tuple_value') {
        to = borrowed(to, { target: '&str', items: '&[String]' }).replaceAll('items.clone()', 'items').replaceAll('text.clone()', 'text').replace(/\(\(items\.len\(\) as f64\) == \(1f64\)\)/u, 'items.len() == 1');
      } else if (definition.name === 'render_tuple_projection') to = borrowed(to, { target: '&str', object: '&str', index: '&str' });
      else if (definition.name === 'accept_tuple_copy_type') to = borrowed(to, { kind: '&str', fields: '&[bool]' }).replaceAll('fields.clone()', 'fields').replaceAll('ml_s1.clone()', 'ml_s1');
      else if (definition.name === 'render_java_script_unit') to = borrowed(to, { language: '&str' });
      else if (definition.name === 'ml_join_product_items_loop1') {
        to = borrowed(to, { items: '&[String]' }).replaceAll('&items,', 'items,');
        for (const name of ['text', 'text_2', 'text_4', 'ml_present']) to = to.replaceAll(`${name}.clone()`, name);
        to = to.replace(/match (crate::translation::product_emission_rules::ml_array::optional_at\([\s\S]*?\)) \{\s*Some\(ml_present\) => ml_present,\s*None => String::from\("undefined"\),\s*\}/u, '$1.unwrap_or_else(|| String::from("undefined"))');
      } else if (definition.name === 'ml_accept_tuple_copy_type_loop2') {
        to = borrowed(to, { fields: '&[bool]' }).replaceAll('&fields,', 'fields,');
        to = to.replace(/match (crate::translation::product_emission_rules::ml_array::optional_at\([\s\S]*?\)) \{\s*Some\(ml_present\) => ml_present,\s*None => false,\s*\}/u, '$1.unwrap_or_default()');
      }
      to = to.replaceAll('String::from("")', 'String::new()');
    } else if (family === 'source') {
      if (definition.name === 'round') to = to.replace('x < 0.0 && x >= -0.5', '(-0.5..0.0).contains(&x)');
      if (definition.name === 'solve_product_source_facts' || definition.name === 'ml_solve_product_source_facts_loop3') {
        to = borrowed(to, { bases: '&[bool]', targets: '&[f64]', sources: '&[f64]', conjunctions: '&[bool]' });
        to = to.replaceAll('let flags = bases.clone();', 'let flags = bases.to_vec();');
        for (const name of ['bases', 'targets', 'sources', 'conjunctions']) {
          to = to.replaceAll(`${name}.clone()`, name).replace(new RegExp(`\\s*let ${name} = ${name};`, 'gu'), '');
        }
        for (const name of ['flags', 'flags_2', 'ml_s1']) to = to.replaceAll(`${name}.clone()`, name);
        to = to.replace('let ml_values3 = previous.clone();', 'let ml_values3 = previous.as_slice();').replace('let previous = previous.clone();', 'let previous = previous.as_slice();');
        to = to.replaceAll('previous.clone(),', 'previous,');
        to = to.replace(/match (self::ml_array::optional_at\(&conjunctions, self::ml_array::number_index\(index\)\)) \{ Some\(ml_present\) => ml_present, None => false \}/u, '$1.unwrap_or_default()');
        to = to.replaceAll('&conjunctions,', 'conjunctions,');
        to = to.replaceAll('.cloned()', '.copied()');
      } else if (definition.name === 'ml_closure1_loop2') {
        to = borrowed(to, { targets: '&[f64]', previous: '&[bool]', sources: '&[f64]' });
        for (const name of ['targets', 'previous', 'sources']) to = to.replaceAll(`&${name},`, `${name},`);
        to = to.replace(/match (self::ml_array::optional_at\([\s\S]*?\)) \{\s*Some\(ml_present\) => ml_present,\s*None => f64::NAN,\s*\}/u, '$1.unwrap_or(f64::NAN)');
      }
    } else throw new Error('unknown product rule family');
    if (definition.name === 'ml_main') to = to.replace('fn ml_main', 'const fn ml_main');
    if (definition.name === 'main') to = to.replace('.spawn(move || ml_main())', '.spawn(ml_main)');
    to = simplifyBranches(to);
    to = simplifyReturns(to);
    if (to.startsWith('pub fn ')) to = '#[must_use]\n' + to;
    if (family === 'emission' && ['ml_join_product_items_loop1', 'ml_accept_tuple_copy_type_loop2'].includes(definition.name)) {
      to = '#[expect(clippy::cast_precision_loss, reason = "the checked JavaScript array-length domain is at most 2^32 - 1, and its loop index starts at zero, advances by one, and is bounded by that length")]\n' + to;
    }
    if (to !== from) {
      actions.push({ op: 'replace', field: 'source', from, to });
      source = source.slice(0, definition.start) + to + source.slice(definition.end);
    }
  }
  if (family === 'source') actions.push({ op: 'replace', field: 'source', from: '/// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.', to: '/// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.' });
  return actions;
}
