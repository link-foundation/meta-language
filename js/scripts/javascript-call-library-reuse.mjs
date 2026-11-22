import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { readProjectData } from './project-data.mjs';
import { readNativeSourceReceipt } from './source-library-native-receipt.mjs';
import {
  nominalReferenceMatchSharingModel, readNominalReferenceMatchSharingInputs,
} from './source-boolean-match-sharing.mjs';

function requireFile(root, { path, bytes, sha256 }) {
  const contents = readFileSync(new URL(path, root));
  if (contents.length !== bytes || createHash('sha256').update(contents).digest('hex') !== sha256) {
    throw new TypeError(`regenerate the changed source authoring output ${path}`);
  }
  return contents;
}

export function requireCurrentJavaScriptCallLibrary(root, stem) {
  if (!['javascript-call-machine', 'javascript-call-source'].includes(stem)) {
    throw new TypeError('unknown JavaScript call library');
  }
  const directory = 'parity/self-translation/';
  const recipe = readNativeSourceReceipt(new URL(directory + stem + '-native.lino', root),
    ['files', 'constructorMatchSharing']);
  if (stem === 'javascript-call-machine') {
    const proof = recipe.constructorMatchSharing;
    const inputs = proof?.producer?.inputs;
    const expected = readNominalReferenceMatchSharingInputs();
    if (proof?.model !== nominalReferenceMatchSharingModel || inputs?.length !== expected.length
      || expected.some(({ path, bytes, sha256 }, index) => inputs[index]?.path !== path
        || inputs[index]?.bytes !== bytes || inputs[index]?.sha256 !== sha256)) {
      throw new TypeError('regenerate the call machine with its current immutable projection inputs');
    }
    if (!recipe.decoratorAssets?.length) {
      throw new TypeError('regenerate the call machine with its ordered native style assets');
    }
  }
  if (recipe.schemaVersion !== 1 || !recipe.sourceClosure?.inputs?.length || !recipe.files?.length
    || !recipe.programProjection?.entryPoints?.length || !recipe.programAsset) {
    throw new TypeError(`regenerate the complete actual source and linked-model receipts for ${stem}`);
  }
  const decorationFile = new RegExp(`^${directory}${stem}-decorators(?:-[1-9][0-9]*)?\\.lino$`, 'u');
  for (const decoration of recipe.decoratorAssets ?? []) {
    if (!decorationFile.test(decoration.path)) throw new TypeError('invalid native style asset path');
    requireFile(root, decoration);
  }
  for (const { path, source } of recipe.sourceClosure.inputs) {
    if (!/^js\/src\/(?:[a-z0-9-]+\/)*[a-z0-9-]+\.js$/u.test(path)
      || readFileSync(new URL(path, root), 'utf8') !== source) {
      throw new TypeError(`source library input changed at ${path}; regenerate ${stem}`);
    }
  }
  const native = stem.replaceAll('-', '_');
  const nativeFile = new RegExp(`^${native}(?:/[a-z][a-z_0-9]*)*\\.rs$`, 'u');
  for (const { path, source } of recipe.files) {
    if (!nativeFile.test(path) || readFileSync(new URL('rust/src/translation/' + path, root), 'utf8') !== source) {
      throw new TypeError(`source library output changed at ${path}; regenerate ${stem}`);
    }
  }
  const asset = recipe.programAsset;
  if (asset.path !== 'js/src/data/' + stem + '-program.lino'
    || !requireFile(root, asset).equals(readFileSync(new URL(asset.path.replace('js/src/', 'rust/src/'), root)))) {
    throw new TypeError(`regenerate the current paired checked source program for ${stem}`);
  }
  const codec = readProjectData(new URL(directory + stem + '-model-codecs.lino', root));
  const entry = `rust/tests/unit/${native}_codecs.rs`;
  const codecFiles = codec.files ?? [{ path: entry, bytes: codec.bytes, sha256: codec.sha256 }];
  const jsCodec = `experiments/javascript-properties/${stem.slice('javascript-'.length)}-codecs.mjs`;
  if (codec.schemaVersion !== 1 || codec.output !== entry || codecFiles.length === 0
    || codecFiles[0].path !== entry || codecFiles[0].bytes !== codec.bytes || codecFiles[0].sha256 !== codec.sha256
    || codec.javascript?.output !== jsCodec) {
    throw new TypeError(`regenerate the current model transport receipt for ${stem}`);
  }
  for (const [index, file] of codecFiles.entries()) {
    if (index && file.path !== `rust/tests/unit/${native}_codecs/part_${index}.rs`) {
      throw new TypeError('invalid generated Rust codec part path');
    }
    requireFile(root, file);
  }
  requireFile(root, { ...codec.javascript, path: jsCodec });
}
