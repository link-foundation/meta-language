// Instantiate the complete generated portable source as a closed lambda.
// Actual receiver and index expressions stay outside its lexical binders.
import { readRuntimeProjectData } from '../project-data-loader.js';

let stringCodeUnitTemplatePolicy;

export function createStringCodeUnitTemplateExpression(span) {
  const policy = stringCodeUnitTemplatePolicy ??= readRuntimeProjectData(new URL('./data/string-code-unit-template.lino', import.meta.url));
  if (policy.schemaVersion !== 1 || policy.model !== 'portable UTF16 code unit lambda 1' || policy.template?.k !== 'lambda') {
    throw new TypeError('the generated UTF-16 code unit template is malformed');
  }
  const instantiate = value => {
    if (Array.isArray(value)) return value.map(instantiate);
    if (value && typeof value === 'object') {
      const result = Object.fromEntries(Object.entries(value).map(([key, child]) => [key, instantiate(child)]));
      if (result.k || Object.hasOwn(result, 'span')) result.span = span;
      return result;
    }
    return value;
  };
  return instantiate(policy.template);
}
