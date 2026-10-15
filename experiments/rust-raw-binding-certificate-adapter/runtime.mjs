import { readProjectData } from '../../js/scripts/project-data.mjs';
import { createCheckedModuleOverlay } from '../rust-text-certificate-adapter/overlay.mjs';

export async function createRustRawBindingCertificateOverlay() {
  const sourceRoot = new URL('../../js/src/', import.meta.url);
  const virtual = new URL('rust-raw-binding-certificate-adapter.js', sourceRoot).href;
  const overlay = await createCheckedModuleOverlay({
    records: readProjectData(new URL('./data/overlays.lino', import.meta.url)), sourceRoot,
    virtualModules: new Map([[virtual, new URL('./certificate.mjs', import.meta.url).href]]),
  });
  try {
    return { ...overlay,
      api: await overlay.importModule('typed-program-links.js'),
      javascript: await overlay.importModule('translation/emit-javascript.js'),
      rust: await overlay.importModule('translation/emit-rust.js'),
      validation: await overlay.importModule('typed-program-links-validation.js'),
      projection: await overlay.importModule('typed-program-checked-projection.js'),
    };
  } catch (error) { overlay.dispose(); throw error; }
}
