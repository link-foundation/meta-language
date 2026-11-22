import { option, tuple } from './types.js';

/**
 * A one-element product keeps own-property presence separate from the value's
 * JavaScript absence. In particular, the empty carrier and a carrier holding
 * undefined describe different object properties.
 */
export function javaScriptOptionalConstructorFieldType(valueType) {
  return option(tuple([option(valueType, 'undefined')]), 'undefined');
}

export function javaScriptConstructorFieldValueType(field) {
  return field.optional ? field.type.element.elements[0] : field.type;
}

export function isJavaScriptOptionalConstructorFieldType(type) {
  const product = type?.element;
  const value = product?.elements?.[0];
  return type?.kind === 'option' && type.absence === 'undefined'
    && product?.kind === 'tuple' && product.elements?.length === 1
    && value?.kind === 'option' && ['undefined', 'both'].includes(value.absence);
}

export function constructJavaScriptOwnedConstructorField(value, type) {
  return { k: 'some', value: { k: 'tuple', items: [value], type: type.element }, type };
}

export function readJavaScriptOptionalConstructorField(carrier, span) {
  return {
    k: 'tupleIndex', index: 0,
    object: {
      k: 'coalesce', left: carrier,
      right: { k: 'tuple', items: [{ k: 'none', absence: 'undefined', span }], span },
      span,
    },
    span,
  };
}
