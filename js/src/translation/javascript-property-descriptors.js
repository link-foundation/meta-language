import {readJavaScriptSemanticValueError,testJavaScriptSemanticSameValue,testJavaScriptSemanticAccessor} from './javascript-semantic-values.js';

/** A normalized Property Descriptor keeps field presence independent of value.
 * Its values belong to one checked semantic-value/global-identity context.
 * @typedef {{ configurablePresent:boolean, configurable:boolean,
 * enumerablePresent:boolean, enumerable:boolean, valuePresent:boolean,
 * value:JavaScriptSemanticValue, writablePresent:boolean, writable:boolean,
 * getterPresent:boolean, getter:JavaScriptSemanticValue,
 * setterPresent:boolean, setter:JavaScriptSemanticValue }} JavaScriptPropertyDescriptor
 */

/** @param {JavaScriptPropertyDescriptor} descriptor @returns {string} */
export function readJavaScriptPropertyDescriptorKind(descriptor) {
  const data = descriptor.valuePresent || descriptor.writablePresent;
  const accessor = descriptor.getterPresent || descriptor.setterPresent;
  return data && accessor ? 'invalid' : data ? 'data' : accessor ? 'accessor' : 'generic';
}

/** Validate only fields actually present; absent payloads carry no meaning.
 * Descriptor conversion, getters on the descriptor object, exotic receivers,
 * proxy traps and heap ownership are separate explicit source operations.
 * @param {JavaScriptPropertyDescriptor} descriptor @returns {string} */
export function readJavaScriptPropertyDescriptorError(descriptor) {
  if (readJavaScriptPropertyDescriptorKind(descriptor) === 'invalid') return 'TypeError';
  if (descriptor.valuePresent && readJavaScriptSemanticValueError(descriptor.value) !== '') return 'InvalidSemanticValue';
  if (descriptor.getterPresent && (readJavaScriptSemanticValueError(descriptor.getter) !== '' || !testJavaScriptSemanticAccessor(descriptor.getter))) return 'TypeError';
  if (descriptor.setterPresent && (readJavaScriptSemanticValueError(descriptor.setter) !== '' || !testJavaScriptSemanticAccessor(descriptor.setter))) return 'TypeError';
  return '';
}

/** @param {JavaScriptPropertyDescriptor} descriptor @returns {boolean} */
export function testCompleteJavaScriptPropertyDescriptor(descriptor) {
  if (!descriptor.configurablePresent || !descriptor.enumerablePresent || readJavaScriptPropertyDescriptorError(descriptor) !== '') return false;
  const kind = readJavaScriptPropertyDescriptorKind(descriptor);
  return kind === 'data' ? descriptor.valuePresent && descriptor.writablePresent
    : kind === 'accessor' && descriptor.getterPresent && descriptor.setterPresent;
}

/** @param {string} kind @param {boolean} configurable @param {boolean} enumerable
 * @returns {JavaScriptPropertyDescriptor} */
export function createJavaScriptPropertyDescriptor(kind, configurable, enumerable) {
  return {configurablePresent:true,configurable,enumerablePresent:true,enumerable,
    valuePresent:kind !== 'accessor',value:{$:'undefined'},writablePresent:kind !== 'accessor',writable:false,
    getterPresent:kind === 'accessor',getter:{$:'undefined'},setterPresent:kind === 'accessor',setter:{$:'undefined'}};
}

/** Completion and field application are functional, so rejection never changes
 * an existing descriptor or invokes a proposed accessor. A kind conversion
 * resets only the inapplicable data/accessor attributes.
 * @param {JavaScriptPropertyDescriptor} base @param {JavaScriptPropertyDescriptor} proposed
 * @returns {JavaScriptPropertyDescriptor} */
export function applyJavaScriptPropertyDescriptorFields(base, proposed) {
  return {
    configurablePresent:true,configurable:proposed.configurablePresent ? proposed.configurable : base.configurable,
    enumerablePresent:true,enumerable:proposed.enumerablePresent ? proposed.enumerable : base.enumerable,
    valuePresent:base.valuePresent,value:base.valuePresent && proposed.valuePresent ? proposed.value : base.value,
    writablePresent:base.writablePresent,writable:base.writablePresent && proposed.writablePresent ? proposed.writable : base.writable,
    getterPresent:base.getterPresent,getter:base.getterPresent && proposed.getterPresent ? proposed.getter : base.getter,
    setterPresent:base.setterPresent,setter:base.setterPresent && proposed.setterPresent ? proposed.setter : base.setter,
  };
}

/** @param {JavaScriptPropertyDescriptor} descriptor @returns {string[]} */
export function readJavaScriptPropertyDescriptorWrites(descriptor) {
  let writes = [];
  if (descriptor.configurablePresent) writes = writes.concat(['configurable']);
  if (descriptor.enumerablePresent) writes = writes.concat(['enumerable']);
  if (descriptor.valuePresent) writes = writes.concat(['value']);
  if (descriptor.writablePresent) writes = writes.concat(['writable']);
  if (descriptor.getterPresent) writes = writes.concat(['get']);
  if (descriptor.setterPresent) writes = writes.concat(['set']);
  return writes;
}

/** @param {string} disposition @param {string} error
 * @param {JavaScriptPropertyDescriptor} descriptor @param {string[]} writes
 * @returns {{disposition:string,error:string,descriptor:JavaScriptPropertyDescriptor,writes:string[]}} */
export function createJavaScriptPropertyTransition(disposition, error, descriptor, writes) {
  return {disposition,error,descriptor,writes};
}

/** Ordinary ValidateAndApplyPropertyDescriptor as an inspectable transition.
 * `keep` is a real no-write operation: an accepted SameValue comparison on a
 * nonconfigurable/nonwritable property must not rewrite its NaN payload.
 * `reject` with empty error is incompatibility, distinct from invalid inputs.
 * The heap adapter performs only the selected create/convert/update/keep
 * operation and listed attribute writes. Unlisted values retain their exact
 * original storage, including NaN payloads absent from a generic proposal.
 * This kernel neither executes descriptor getters nor claims a full heap model.
 * @param {boolean} exists @param {boolean} extensible
 * @param {JavaScriptPropertyDescriptor} current @param {JavaScriptPropertyDescriptor} proposed
 * @returns {{disposition:string,error:string,descriptor:JavaScriptPropertyDescriptor,writes:string[]}} */
export function planJavaScriptPropertyDefinition(exists, extensible, current, proposed) {
  const error = readJavaScriptPropertyDescriptorError(proposed);
  if (error !== '') return createJavaScriptPropertyTransition('reject', error, current, []);
  if (exists && !testCompleteJavaScriptPropertyDescriptor(current)) return createJavaScriptPropertyTransition('reject', 'InvalidCurrentDescriptor', current, []);
  const kind = readJavaScriptPropertyDescriptorKind(proposed);
  if (!exists) {
    if (!extensible) return createJavaScriptPropertyTransition('reject', '', current, []);
    const base = createJavaScriptPropertyDescriptor(kind, false, false);
    const descriptor = applyJavaScriptPropertyDescriptorFields(base, proposed);
    return createJavaScriptPropertyTransition('create', '', descriptor, readJavaScriptPropertyDescriptorWrites(descriptor));
  }
  if (!proposed.configurablePresent && !proposed.enumerablePresent && !proposed.valuePresent
    && !proposed.writablePresent && !proposed.getterPresent && !proposed.setterPresent)
    return createJavaScriptPropertyTransition('keep', '', current, []);
  const original = readJavaScriptPropertyDescriptorKind(current);
  if (!current.configurable) {
    if (proposed.configurablePresent && proposed.configurable) return createJavaScriptPropertyTransition('reject', '', current, []);
    if (proposed.enumerablePresent && proposed.enumerable !== current.enumerable) return createJavaScriptPropertyTransition('reject', '', current, []);
    if (kind !== 'generic' && kind !== original) return createJavaScriptPropertyTransition('reject', '', current, []);
    if (original === 'accessor') {
      if (proposed.getterPresent && !testJavaScriptSemanticSameValue(proposed.getter, current.getter)) return createJavaScriptPropertyTransition('reject', '', current, []);
      if (proposed.setterPresent && !testJavaScriptSemanticSameValue(proposed.setter, current.setter)) return createJavaScriptPropertyTransition('reject', '', current, []);
    } else if (!current.writable) {
      if (proposed.writablePresent && proposed.writable) return createJavaScriptPropertyTransition('reject', '', current, []);
      if (proposed.valuePresent) return createJavaScriptPropertyTransition(testJavaScriptSemanticSameValue(proposed.value, current.value) ? 'keep' : 'reject', '', current, []);
    }
  }
  const base = kind !== 'generic' && kind !== original
    ? createJavaScriptPropertyDescriptor(kind, current.configurable, current.enumerable) : current;
  const descriptor = applyJavaScriptPropertyDescriptorFields(base, proposed);
  const conversion = kind !== 'generic' && kind !== original;
  return createJavaScriptPropertyTransition(conversion ? 'convert' : 'update', '', descriptor,
    readJavaScriptPropertyDescriptorWrites(conversion ? descriptor : proposed));
}
