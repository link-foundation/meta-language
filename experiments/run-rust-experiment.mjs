#!/usr/bin/env node
// Runs an experiment's Rust program against the repository's meta-language
// crate without keeping a Cargo package in the tree: the program is copied
// into a scratch crate under the system temporary directory (so no manifest,
// lockfile or build output lands in experiments/ and the dependency inventory
// stays about the shipped packages), built there with the crate's own
// Cargo.lock, and run from the caller's working directory.
//
//   node experiments/run-rust-experiment.mjs <program.rs> [arguments...]
import { execFileSync } from 'node:child_process';
import { copyFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const [program, ...args] = process.argv.slice(2);
if (!program) {
  console.error('usage: node experiments/run-rust-experiment.mjs <program.rs> [arguments...]');
  process.exit(2);
}
const crate = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', 'rust');
const name = path.basename(path.dirname(path.resolve(program)));
const scratch = path.join(tmpdir(), 'meta-language-experiments', name);
mkdirSync(path.join(scratch, 'src'), { recursive: true });
copyFileSync(path.resolve(program), path.join(scratch, 'src', 'main.rs'));
copyFileSync(path.join(crate, 'Cargo.lock'), path.join(scratch, 'Cargo.lock'));
writeFileSync(
  path.join(scratch, 'Cargo.toml'),
  `[package]\nname = "${name}"\nversion = "0.0.0"\nedition = "2024"\npublish = false\n\n[workspace]\n\n` +
    `[dependencies]\nmeta-language = { path = ${JSON.stringify(crate)}, default-features = false }\nserde_json = "1"\n`,
);
const binary = path.join(scratch, 'target', 'release', name);
execFileSync('cargo', ['build', '--quiet', '--release', '--manifest-path', path.join(scratch, 'Cargo.toml')], { stdio: 'inherit' });
process.stdout.write(execFileSync(binary, args, { maxBuffer: 256 * 1024 * 1024 }));
