// The schema captured in the shared Links lexical-rule policy. Native form
// tables are generated from these exact fields; there is no second schema.
export const GRAMMAR_LEXER_FORMS = Object.freeze({
  lexicalScope: [['name', 'name'], ['owner', 'name'], ['item', 'expression']],
  lexicalToken: [['name', 'name'], ['token', 'name']],
  lexicalOwner: [['name', 'name'], ['item', 'expression']],
  lexicalBinding: [['name', 'name'], ['item', 'expression']],
  lexicalLiteral: [['value', 'text'], ['name', 'name']],
  lexicalRule: [['name', 'name'], ['type', 'name'], ['mode', 'name'], ['channel', 'name'], ['disposition', 'name'], ['item', 'expression'], ['action', 'block:action']],
  lexicalFragment: [['name', 'name'], ['item', 'expression']],
  lexicalObligation: [['name', 'name'], ['category', 'name'], ['source', 'text']],
  lexicalChannel: [['name', 'name']],
  lexicalMode: [['name', 'name']],
  lexicalType: [['name', 'name']],
});
