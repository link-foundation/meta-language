#!/usr/bin/env node
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { formatProductRuleSource, readProductRuleStyleCorrections } from './product-rule-style-corrections.mjs';

const input = new URL('../src/translation/product-emission-rules.js', import.meta.url);
const output = new URL('../../rust/src/translation/product_emission_rules.rs', import.meta.url);
const style = new URL('../../parity/self-translation/product-emission-rules-decorators.lino', import.meta.url);
let decorators = DecoratorSet.fromLino(readFileSync(style, 'utf8'));
const reviewedId = 'reviewed-product-native-lints-285c8b13';
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'))));
const base = new DecoratorSet(decorators.decorators.filter(definition => definition.id !== reviewedId));
const adapted = decorateEmitted('Rust', { source: translated.text }, base).source;
const formatted = formatProductRuleSource(adapted);
const actions = readProductRuleStyleCorrections(formatted, 'emission', readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8'));
if (process.argv.includes('--update-styles')) {
  decorators = base.add({ id: reviewedId, level: 'emitter', order: 1000, when: [['format', 'Rust'], ['scope', 'formatted-product-library']], actions });
  writeFileSync(style, decorators.toLino());
}
const recorded = decorators.decorators.find(definition => definition.id === reviewedId);
if (JSON.stringify(recorded?.actions) !== JSON.stringify(actions)) throw new Error('reviewed product emission styles are stale; regenerate with --update-styles');
const reviewed = new DecoratorSet([recorded]).decorate('emitter', { format: 'Rust', scope: 'formatted-product-library', source: formatted }).source;
const text = '// Generated from js/src/translation/product-emission-rules.js by\n// js/scripts/generate-product-emission-rules.mjs. Do not edit by hand.\n' + formatProductRuleSource(reviewed);
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('product emission rules are stale; run node js/scripts/generate-product-emission-rules.mjs');
  console.log('generated Rust product emission rules match JavaScript');
} else writeFileSync(output, text);
