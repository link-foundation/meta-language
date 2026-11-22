const sourceOwnerDirectories = new WeakMap();

/** A scoped directory for roots produced by the source closure checker. */
export function withSourceOwnerContainers(root, directory, check) {
  const previous = sourceOwnerDirectories.get(root);
  sourceOwnerDirectories.set(root, directory);
  try {
    return check();
  } finally {
    if (previous) sourceOwnerDirectories.set(root, previous);
    else sourceOwnerDirectories.delete(root);
  }
}

/** Check represented host containers without invoking accessors or iterator overrides. */
export function checkOwnedProgramContainers(root, { acceptContainer, fail, selectedFields }) {
  if (!root || typeof root !== 'object' || Array.isArray(root)
    || ![Object.prototype, null].includes(Object.getPrototypeOf(root))) {
    fail('the owned program root must be a plain data record');
  }
  const directory = sourceOwnerDirectories.get(root);
  const rootState = createState();
  const ownedStates = directory && new Map();
  const rootItems = directory && Object.getOwnPropertyDescriptor(root, 'items')?.value;
  const rootDeclarations = directory && Object.getOwnPropertyDescriptor(root, 'declarations')?.value;
  let selected = false;
  let defect;
  function createState() {
    return {
      visited: new WeakSet(),
      inspectedPrototypes: new WeakSet(),
      visits: 0,
      prototypeVisits: 0,
    };
  }
  const reject = detail => { defect ??= detail; };
  function ownedState(value) {
    const span = value && Object.getOwnPropertyDescriptor(value, 'span')?.value;
    const start = span && Object.getOwnPropertyDescriptor(span, 'start')?.value;
    const end = span && Object.getOwnPropertyDescriptor(span, 'end')?.value;
    const owner = Number.isSafeInteger(start) && Number.isSafeInteger(end) && start <= end
      && directory.ranges.find(range => range.start <= start && start < range.end && end <= range.end);
    if (!owner) fail('a source library declaration has no complete original module interval');
    if (!ownedStates.has(owner)) ownedStates.set(owner, createState());
    return ownedStates.get(owner);
  }
  function inspectPrototypes(value, state) {
    let depth = 0;
    const { inspectedPrototypes } = state;
    for (let prototype = Object.getPrototypeOf(value); prototype && !inspectedPrototypes.has(prototype);
      prototype = Object.getPrototypeOf(prototype)) {
      if (++depth > 256 || ++state.prototypeVisits > 100_000) {
        fail('the owned prototype graph exceeds its resource limit');
      }
      inspectedPrototypes.add(prototype);
      if (selectedFields.some(field => Object.getOwnPropertyDescriptor(prototype, field))) {
        selected = true;
        reject('a source marker is inherited instead of represented as an own field');
      }
    }
  }
  function visit(value, depth, state) {
    const { visited } = state;
    if (!value || typeof value !== 'object' && typeof value !== 'function' || visited.has(value)) return;
    if (depth > 256 || ++state.visits > 100_000) {
      fail('the owned container graph exceeds its resource limit (depth ' + depth + ', visited ' + state.visits + ')');
    }
    visited.add(value);
    const keys = Reflect.ownKeys(value);
    if (keys.some(key => selectedFields.includes(key))) selected = true;
    inspectPrototypes(value, state);
    if (value instanceof Map) {
      if (!acceptContainer('map', true, keys.length > 0 || Object.getPrototypeOf(value) !== Map.prototype)) reject('a declaration Map has unrepresented own fields');
      for (const [key, child] of Map.prototype.entries.call(value)) {
        visit(key, depth + 1, state);
        visit(child, depth + 1, value === rootDeclarations ? ownedState(child) : state);
      }
    } else if (Array.isArray(value)) {
      const length = Object.getOwnPropertyDescriptor(value, 'length').value;
      const dense = keys.length === length + 1 && keys.every(key => key === 'length' || typeof key === 'string'
        && /^(?:0|[1-9][0-9]*)$/u.test(key) && Number(key) < length && String(Number(key)) === key);
      const unrepresented = Object.getPrototypeOf(value) !== Array.prototype || keys.some(key => {
        const descriptor = Object.getOwnPropertyDescriptor(value, key);
        return !Object.hasOwn(descriptor, 'value') || key !== 'length' && !descriptor.enumerable;
      });
      if (!acceptContainer('array', dense, unrepresented)) reject('an ordered container has unrepresented own fields or sparse positions');
      for (const key of keys) if (key !== 'length') {
        const descriptor = Object.getOwnPropertyDescriptor(value, key);
        if (Object.hasOwn(descriptor, 'value')) {
          const child = descriptor.value;
          visit(child, depth + 1, value === rootItems ? ownedState(child) : state);
        }
      }
    } else {
      const unrepresented = ![Object.prototype, null].includes(Object.getPrototypeOf(value)) || keys.some(key => {
        const descriptor = Object.getOwnPropertyDescriptor(value, key);
        return typeof key !== 'string' || !descriptor.enumerable || !Object.hasOwn(descriptor, 'value');
      });
      if (!acceptContainer('record', true, unrepresented)) reject('an owned record has unrepresented own fields or a prototype');
      for (const key of keys) {
        const descriptor = Object.getOwnPropertyDescriptor(value, key);
        if (Object.hasOwn(descriptor, 'value')) visit(descriptor.value, depth + 1, state);
      }
    }
  }
  visit(root, 0, rootState);
  if (selected && defect) fail(defect);
  return selected;
}
