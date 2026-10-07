import { execFileSync } from 'node:child_process';

// Compile the Rust side of a shared case without changing its source bytes.
export function compileRustTranslation(sourceLanguage, source, metadata, execute = execFileSync) {
  const generated = sourceLanguage !== 'Rust';
  execute(generated ? 'clippy-driver' : 'rustc', [
    '--edition', '2024', '--crate-type', 'lib', '--emit', 'metadata',
    ...(generated ? ['-D', 'warnings'] : []), '-o', metadata, source,
  ], { stdio: 'pipe' });
}
