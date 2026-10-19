// Exact fifth-stage append. Every published predecessor remains immutable.
// Collection constructors retain one evaluated source operand and their full type.
export const ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS = {
  expressionProjections: {
    set: {construct: 'set', fields: [['items', 'expression', 'items'], ['ty', 'copy', 'type']]},
    map: {construct: 'map', fields: [['entries', 'expression', 'entries'], ['ty', 'copy', 'type']]},
  },
};
export const ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_SCOPE = 'Fifth append-only Set and Map constructor projection source closure only; no native execution, raw producer replacement or whole-language coverage claim.';
