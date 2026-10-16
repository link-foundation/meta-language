// Selection is grammar data. Candidate names qualify whole grammar definitions;
// they are never inferred from a host language pair or from matching samples.
export const GRAMMAR_SELECTION_FORMS = Object.freeze({
  'any-grammar': [['item', 'expression'], ['items', 'expressions']],
  'all-grammar': [['item', 'expression'], ['items', 'expressions']],
  'grammar-candidate': [['name', 'name'], ['mapping', 'name']],
});
