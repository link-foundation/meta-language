// Offline inventory of immutable grammar downloads. Acquiring their bytes is
// separate: deleting a temporary cache never deletes a declared dependency.

const hash = (value, description) => {
  if (typeof value !== 'string' || !/^[a-f0-9]{64}$/u.test(value)) {
    throw new Error(`${description} does not record an exact sha256`);
  }
  return value;
};
const pin = (value, description) => {
  if (typeof value !== 'string' || !value.trim() || /^(?:latest|main|master|HEAD)$/u.test(value)) {
    throw new Error(`${description} does not record an immutable pin`);
  }
  return value;
};
const grammarMetadata = (grammar, id) => {
  if (grammar.id !== id) throw new Error(`grammar ${id} has inconsistent identity ${grammar.id}`);
  pin(grammar.version, `grammar ${id}`);
  hash(grammar.parserSha256, `grammar ${id} parser`);
  hash(grammar.wasmSha256, `grammar ${id} WebAssembly`);
  if (grammar.oracle !== undefined && typeof grammar.oracle !== 'boolean') throw new Error(`grammar ${id} has invalid oracle provenance`);
  if (grammar.patchSha256 !== undefined) hash(grammar.patchSha256, `grammar ${id} patch`);
  if (grammar.revision !== undefined) pin(grammar.revision, `grammar ${id} source revision`);
  return Object.fromEntries(['id', 'crate', 'directory', 'upstream', 'vendored', 'revision', 'version',
    'parserSha256', 'wasmSha256', 'license', 'oracle', 'patch', 'patchSha256']
    .filter((field) => grammar[field] !== undefined).map((field) => [field, grammar[field]]));
};

/** Collect pins and provenance without opening or downloading any grammar asset. */
export function collectGrammarAssetDependencies({ grammarLock, runtimeLock, grammarLockFile,
  rustGrammarLockFile, runtimeLockFile, crateItems, grammarFile, addItem }) {
  if (!grammarLock || !grammarLock.grammars || typeof grammarLock.grammars !== 'object'
    || Array.isArray(grammarLock.grammars)) throw new Error('the grammar lock has no grammar records');
  const cli = /^tree-sitter (\d+\.\d+\.\d+)$/u.exec(grammarLock.treeSitterCli)?.[1];
  if (!cli) throw new Error('the grammar lock does not pin an exact tree-sitter CLI');
  const cliItem = addItem({
    category: 'toolchain', name: 'tree-sitter-cli', declaredIn: [grammarLockFile, rustGrammarLockFile],
    pinned: cli, compare: 'version', source: { type: 'npm', package: 'tree-sitter-cli' },
  });
  for (const [id, grammar] of Object.entries(grammarLock.grammars).sort(([a], [b]) => a.localeCompare(b))) {
    const provenance = grammarMetadata(grammar, id);
    let parent;
    if (grammar.vendored) {
      pin(grammar.upstream, `grammar ${id} repository`);
      parent = addItem({
        category: 'downloaded-parser', name: grammar.upstream,
        declaredIn: [grammarLockFile, `${grammar.vendored}/NOTICE.md`],
        pinned: grammar.version, compare: 'revision',
        source: { type: 'github-revision', repository: grammar.upstream },
        aliases: [`vendored-parser ${grammar.vendored}@${grammar.version}`],
        provenance,
        ...(grammar.patch ? { patch: grammar.patch } : {}),
      });
    } else {
      parent = crateItems.find((item) => item.name === grammar.crate && item.pinned === grammar.version);
      if (!parent) throw new Error(`grammar ${id} names ${grammar.crate} ${grammar.version}, which rust/Cargo.lock does not lock`);
    }
    const legacy = `${grammar.oracle ? 'js/oracles/grammars' : 'js/src/vendor/grammars'}/${id}.wasm.gz`;
    addItem({
      category: 'downloaded-grammar', name: grammarFile(grammar, `${id}.wasm.gz`), declaredIn: grammarLockFile,
      pinned: grammar.version, compare: 'derived', source: { type: 'none' },
      aliases: [`vendored-grammar ${legacy}@${grammar.version}`], provenance,
      dependsOn: [parent.id, cliItem.id],
    });
  }
  pin(runtimeLock.version, 'web-tree-sitter runtime');
  pin(runtimeLock.upstream, 'web-tree-sitter repository');
  pin(runtimeLock.revision, 'web-tree-sitter runtime revision');
  pin(runtimeLock.emscripten, 'web-tree-sitter compiler image');
  hash(runtimeLock.wasmSha256, 'web-tree-sitter runtime');
  hash(runtimeLock.upstreamWasmSha256, 'web-tree-sitter upstream runtime');
  hash(runtimeLock.patchSha256, 'web-tree-sitter runtime patch');
  const image = addItem({
    category: 'image', name: 'emscripten/emsdk',
    declaredIn: [runtimeLockFile, 'js/scripts/build-web-tree-sitter-runtime.mjs'],
    pinned: runtimeLock.emscripten, compare: 'version',
    source: { type: 'docker-hub', repository: 'emscripten/emsdk' },
  });
  // This runtime binary is retained in the repository; it is not a grammar.
  addItem({
    category: 'vendored-runtime', name: 'js/src/vendor/web-tree-sitter/web-tree-sitter.wasm.gz',
    declaredIn: runtimeLockFile, pinned: runtimeLock.version, compare: 'version',
    source: { type: 'github-release', repository: runtimeLock.upstream }, dependsOn: [image.id],
    provenance: Object.fromEntries(['upstream', 'version', 'revision', 'emscripten', 'patch',
      'patchSha256', 'upstreamWasmSha256', 'wasmSha256', 'license']
      .filter((field) => runtimeLock[field] !== undefined).map((field) => [field, runtimeLock[field]])),
    ...(runtimeLock.patch ? { patch: runtimeLock.patch } : {}),
  });
}

/** Follow an explicit old inventory identity only for the same source and pin. */
export function relocatedGrammarDependency(item, previous) {
  const predecessor = { 'downloaded-parser': 'vendored-parser', 'downloaded-grammar': 'vendored-grammar' }[item.category];
  if (!predecessor) return undefined;
  const candidates = (item.aliases ?? []).map((alias) => previous.get(alias)).filter(Boolean);
  if (candidates.length > 1) throw new Error(`${item.id}: ambiguous grammar dependency relocation`);
  const old = candidates[0];
  if (!old) return undefined;
  if (old.category !== predecessor || old.scope !== item.scope || old.pinned !== item.pinned || old.compare !== item.compare
    || JSON.stringify(old.source) !== JSON.stringify(item.source)) {
    throw new Error(`${item.id}: grammar relocation changed its audited source or pin; run a live dependency refresh`);
  }
  return old;
}
