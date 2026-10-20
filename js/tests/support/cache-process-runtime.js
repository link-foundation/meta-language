import {copyFileSync,linkSync} from 'node:fs';

// uutils 0.8.0 selects Linux dispatch from the kernel executable path, so its
// sleep cannot be renamed to a fixture or cargo using argv0:
// https://github.com/uutils/coreutils/blob/0.8.0/src/common/validation.rs#L79
// Use the already-running JavaScript runtime. Hard links consume no duplicate
// binary storage; cross-device fixtures copy. Never chmod the linked runtime.
export function copyRuntimeProcessFixture(file) {
  try { linkSync(process.execPath,file); }
  catch (failure) {
    if(failure.code!=='EXDEV')throw failure;
    copyFileSync(process.execPath,file);
  }
}
