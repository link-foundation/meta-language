import { DecoratorSet } from '../src/decorators.js';
import { readReviewedCompilerCloneStyles } from './reviewed-compiler-clone-styles.mjs';

const file = new URL('../../parity/self-translation/portable-ci-clone-review-138183525811.lino', import.meta.url);
const hash = '69ce59f7a9d4b0bdab03e76a0167b48e2e5ea51a2f95983f6c87ede335fef05c';
const counts = {
  resource_description_rules: 12,
  ascii_text_predicate_rules: 1,
  callable_emission_rules: 1,
  declaration_dependency_rules: 1,
  grammar_lexer_rules: 14,
  javascript_record_parameter_rules: 2,
};
const paths = [
  'rust/src/resource_description_rules/part_1.rs',
  'rust/src/resource_description_rules/part_2.rs',
  'rust/src/translation/ascii_text_predicate_rules/part_2.rs',
  'rust/src/translation/callable_emission_rules/part_2.rs',
  'rust/src/translation/declaration_dependency_rules/part_4.rs',
  'rust/src/translation/grammar_lexer_rules/part_4.rs',
  'rust/src/translation/javascript_record_parameter_rules/part_3.rs',
];
const id = 'portable-compiler-clone-review';

/** Reuse the archived ownership review only on each exact generated function. */
export function applyReviewedPortableCloneStyles(items, family, format) {
  if (!Object.hasOwn(counts, family)) return items;
  const layer = new DecoratorSet([readReviewedCompilerCloneStyles(items, family, { file, hash, counts, paths, id })]);
  return items.map(item => {
    const corrected = layer.decorate('emitter', { format: 'Rust', scope: 'formatted-item', source: item }).source;
    return corrected === item ? item : format(corrected);
  });
}
