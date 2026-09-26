#!/usr/bin/env node
// Installs exact npm and crate candidate artifacts into clean consumers on the
// current platform, parses the shared consumer corpus offline through public
// entry points only, and writes a platform report. The issue 195 evidence
// runner turns these reports into package-delivery execution records, and CI
// runs this script on every supported platform against the same artifacts.
import { createHash } from 'node:crypto';
import { gunzipSync } from 'node:zlib';
import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const corpusPath = path.resolve(option('--corpus', path.join(root, 'parity/fixtures/issue-195-evidence.json')));
const tarball = path.resolve(required('--npm-tarball'));
const crate = path.resolve(required('--crate'));
const workDirectory = path.resolve(required('--work-dir'));
const output = path.resolve(required('--out'));
const corpus = JSON.parse(await readFile(corpusPath, 'utf8'));
const platform = `${process.platform}-${process.arch}`;

// Crates that open network connections; none may be a normal dependency of
// the parse path the consumer exercises.
const NETWORK_CRATES = new Set([
  'async-std', 'attohttpc', 'curl', 'h2', 'hyper', 'isahc', 'mio', 'native-tls',
  'openssl', 'reqwest', 'rustls', 'socket2', 'surf', 'tokio', 'ureq',
]);
const NETWORK_APIS = /\bstd::net\b|\bTcpStream\b|\bTcpListener\b|\bUdpSocket\b|\bToSocketAddrs\b/u;

const logs = [];

async function main() {
  await rm(workDirectory, { recursive: true, force: true });
  await mkdir(workDirectory, { recursive: true });
  const report = {
    schemaVersion: 1,
    platform,
    corpusSha256: sha256(await readFile(corpusPath)),
    node: process.version,
    npm: await npmConsumer(),
    crate: await crateConsumer(),
    logs,
  };
  report.agreement = agreement(report.npm.observations, report.crate.observations);
  await mkdir(path.dirname(output), { recursive: true });
  await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
  console.log(`issue-195 consumer: wrote ${platform} report to ${output}`);
}

async function npmConsumer() {
  const directory = path.join(workDirectory, 'npm-consumer');
  const cache = path.join(workDirectory, 'npm-cache');
  const bytes = await readFile(tarball);
  const checksum = {
    artifact: path.basename(tarball),
    sha256: sha256(bytes),
    expectedSha256: await expectedChecksum(tarball),
  };
  await mkdir(directory, { recursive: true });
  const clean = {
    consumerWasEmpty: (await readdir(directory)).length === 0,
    freshCache: !existsSync(cache),
  };
  await writeFile(path.join(directory, 'package.json'), `${JSON.stringify({
    name: 'issue-195-npm-consumer',
    private: true,
    type: 'module',
  }, null, 2)}\n`);
  const environment = { ...childEnvironment(), npm_config_cache: cache, npm_config_update_notifier: 'false' };
  await run('npm-install', 'npm', [
    'install', '--ignore-scripts', '--no-audit', '--no-fund', tarball,
  ], { cwd: directory, env: environment });
  const lock = JSON.parse(await readFile(path.join(directory, 'package-lock.json'), 'utf8'));
  const installed = lock.packages?.['node_modules/meta-language'] ?? {};
  const installedPackage = JSON.parse(
    await readFile(path.join(directory, 'node_modules/meta-language/package.json'), 'utf8'),
  );
  clean.resolvedFromArtifact = typeof installed.resolved === 'string' && installed.resolved.startsWith('file:');
  clean.onlyArtifactIsDirectDependency =
    JSON.stringify(Object.keys(lock.packages?.['']?.dependencies ?? {})) === '["meta-language"]';
  checksum.installedIntegrity = installed.integrity ?? null;
  checksum.expectedIntegrity = `sha512-${createHash('sha512').update(bytes).digest('base64')}`;
  await writeFile(path.join(directory, 'deny-network.mjs'), DENY_NETWORK);
  await writeFile(path.join(directory, 'consumer.mjs'), NPM_CONSUMER);
  const observationPath = path.join(directory, 'observations.json');
  await run('npm-consumer', process.execPath, [
    '--import', './deny-network.mjs', 'consumer.mjs', corpusPath, observationPath,
  ], { cwd: directory, env: { ...environment, NODE_OPTIONS: '' } });
  const observed = JSON.parse(await readFile(observationPath, 'utf8'));
  return {
    version: installedPackage.version,
    checksum,
    clean,
    publicEntryPoints: observed.publicEntryPoints,
    offline: observed.offline,
    observations: observed.observations,
  };
}

async function crateConsumer() {
  const directory = path.join(workDirectory, 'crate-consumer');
  const extracted = path.join(workDirectory, 'crate-source');
  const target = path.join(workDirectory, 'crate-target');
  const checksum = {
    artifact: path.basename(crate),
    sha256: sha256(await readFile(crate)),
    expectedSha256: await expectedChecksum(crate),
  };
  await mkdir(path.join(directory, 'src'), { recursive: true });
  await mkdir(extracted, { recursive: true });
  const clean = {
    consumerWasEmpty: (await readdir(directory)).join() === 'src' &&
      (await readdir(path.join(directory, 'src'))).length === 0,
    freshTargetDirectory: !existsSync(target),
  };
  await extractCrate(await readFile(crate), extracted);
  const crateSource = path.join(extracted, path.basename(crate).replace(/\.crate$/u, ''));
  const manifest = await readFile(path.join(crateSource, 'Cargo.toml'), 'utf8');
  const version = /^version = "([^"]+)"/mu.exec(manifest)?.[1] ?? null;
  await writeFile(path.join(directory, 'Cargo.toml'), [
    '[package]',
    'name = "issue-195-crate-consumer"',
    'version = "0.0.0"',
    'edition = "2024"',
    '',
    '[dependencies]',
    `meta-language = { path = ${JSON.stringify(crateSource.split(path.sep).join('/'))}, default-features = false }`,
    'serde_json = "1"',
    '',
  ].join('\n'));
  await writeFile(path.join(directory, 'src/main.rs'), CRATE_CONSUMER);
  const environment = { ...childEnvironment(), CARGO_TARGET_DIR: target };
  await run('crate-fetch', 'cargo', ['fetch'], { cwd: directory, env: environment });
  const offlineEnvironment = {
    ...environment,
    CARGO_NET_OFFLINE: 'true',
    HTTP_PROXY: 'http://127.0.0.1:9',
    HTTPS_PROXY: 'http://127.0.0.1:9',
    http_proxy: 'http://127.0.0.1:9',
    https_proxy: 'http://127.0.0.1:9',
    NO_PROXY: '',
    no_proxy: '',
  };
  await run('crate-build', 'cargo', ['build', '--offline', '--quiet'], {
    cwd: directory,
    env: offlineEnvironment,
  });
  const tree = await run('crate-tree', 'cargo', [
    'tree', '--offline', '-e', 'normal', '--prefix', 'none', '--format', '{p}',
  ], { cwd: directory, env: offlineEnvironment });
  const dependencies = [...new Set(tree.split(/\r?\n/u).map((line) => line.split(' ')[0]).filter(Boolean))];
  const lock = await readFile(path.join(directory, 'Cargo.lock'), 'utf8');
  const lockEntry = /\[\[package\]\]\nname = "meta-language"\nversion = "([^"]+)"\n(source = [^\n]+\n)?/u.exec(lock);
  clean.resolvedFromArtifact = Boolean(lockEntry) && !lockEntry[2] && lockEntry[1] === version;
  await mkdir(path.join(directory, 'examples'), { recursive: true });
  await writeFile(
    path.join(directory, 'examples/private_path.rs'),
    `#[allow(unused_imports)]\nuse ${corpus.delivery.privateRustPath} as _;\nfn main() {}\n`,
  );
  const privateCheck = await run('crate-private-path', 'cargo', [
    'check', '--offline', '--quiet', '--example', 'private_path',
  ], { cwd: directory, env: offlineEnvironment, expectFailure: true });
  const privatePathRejected = /error\[E0603\]/u.test(privateCheck);
  const observationPath = path.join(directory, 'observations.json');
  const binary = path.join(target, 'debug', `issue-195-crate-consumer${process.platform === 'win32' ? '.exe' : ''}`);
  await run('crate-consumer', binary, [corpusPath, observationPath], {
    cwd: directory,
    env: offlineEnvironment,
  });
  const observed = JSON.parse(await readFile(observationPath, 'utf8'));
  const sources = await rustSources(path.join(crateSource, 'src'));
  return {
    version,
    checksum,
    clean,
    publicEntryPoints: { ...observed.publicEntryPoints, privatePathRejected },
    offline: {
      builtOffline: true,
      ranWithNetworkDisabledEnvironment: true,
      networkDependencies: dependencies.filter((name) => NETWORK_CRATES.has(name)),
      networkApiFiles: sources
        .filter(({ text }) => NETWORK_APIS.test(text))
        .map(({ file }) => path.relative(crateSource, file).split(path.sep).join('/')),
    },
    observations: observed.observations,
  };
}

function agreement(left, right) {
  const sections = ['parses', 'programs', 'translations'];
  const mismatches = sections.filter((section) =>
    canonical(left?.[section]) !== canonical(right?.[section]));
  return { sections, mismatches };
}

// Extracts a gzip tar archive without an external `tar`, whose flavour and
// drive-letter handling differ across the supported platforms.
async function extractCrate(bytes, destination) {
  const archive = gunzipSync(bytes);
  let longName = null;
  for (let offset = 0; offset + 512 <= archive.length;) {
    const header = archive.subarray(offset, offset + 512);
    if (header.every((byte) => byte === 0)) break;
    const field = (start, length) => header.subarray(start, start + length).toString('utf8').replace(/\0.*$/su, '');
    const size = Number.parseInt(field(124, 12).trim() || '0', 8);
    const type = field(156, 1) || '0';
    const body = archive.subarray(offset + 512, offset + 512 + size);
    offset += 512 + Math.ceil(size / 512) * 512;
    if (type === 'L') {
      longName = body.toString('utf8').replace(/\0.*$/su, '');
      continue;
    }
    if (type === 'x') {
      longName = /\d+ path=([^\n]*)\n/u.exec(body.toString('utf8'))?.[1] ?? longName;
      continue;
    }
    const prefix = field(345, 155);
    const name = longName ?? (prefix ? `${prefix}/${field(0, 100)}` : field(0, 100));
    longName = null;
    const target = path.resolve(destination, name);
    if (!target.startsWith(path.resolve(destination) + path.sep)) {
      throw new Error(`crate entry escapes the extraction directory: ${name}`);
    }
    if (type === '5') {
      await mkdir(target, { recursive: true });
    } else if (type === '0' || type === '7') {
      await mkdir(path.dirname(target), { recursive: true });
      await writeFile(target, body);
    } else if (type !== 'g') {
      throw new Error(`unsupported crate archive entry type ${type} for ${name}`);
    }
  }
}

function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
  if (value && typeof value === 'object') {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(',')}}`;
  }
  return JSON.stringify(value) ?? 'null';
}

async function rustSources(directory) {
  const files = [];
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) files.push(...await rustSources(file));
    else if (entry.name.endsWith('.rs')) files.push({ file, text: await readFile(file, 'utf8') });
  }
  return files;
}

async function expectedChecksum(artifact) {
  const text = await readFile(`${artifact}.sha256`, 'utf8');
  const [digest, name] = text.trim().split(/\s+/u);
  if (name !== path.basename(artifact)) {
    throw new Error(`checksum file names ${name}, expected ${path.basename(artifact)}`);
  }
  return digest;
}

/**
 * The parent environment as a plain object. Windows names are case-insensitive, so a copy can
 * hold both `PATH` and `Path`, and a child such as `cmd.exe` then sees only one of them; the
 * search path is merged into a single `PATH` so every command stays resolvable.
 */
function childEnvironment() {
  const environment = { ...process.env };
  if (process.platform !== 'win32') return environment;
  const names = Object.keys(environment).filter((name) => name.toUpperCase() === 'PATH');
  const entries = names.flatMap((name) => environment[name].split(path.delimiter)).filter(Boolean);
  for (const name of names) delete environment[name];
  environment.PATH = [...new Set(entries)].join(path.delimiter);
  return environment;
}

async function run(label, command, args, { cwd = workDirectory, env = childEnvironment(), expectFailure = false } = {}) {
  const logPath = path.join(workDirectory, `${label}.log`);
  const rendered = [command, ...args].join(' ');
  const { code, stdout, stderr } = await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      env,
      shell: process.platform === 'win32' && ['npm', 'cargo'].includes(command),
    });
    const out = [];
    const err = [];
    child.stdout.on('data', (chunk) => out.push(chunk));
    child.stderr.on('data', (chunk) => err.push(chunk));
    child.once('error', reject);
    child.once('close', (exit) => resolve({
      code: exit,
      stdout: Buffer.concat(out).toString('utf8'),
      stderr: Buffer.concat(err).toString('utf8'),
    }));
  });
  await writeFile(logPath, `command: ${rendered}\ncwd: ${cwd}\n\n${stdout}\n${stderr}\nexit: ${code}\n`);
  logs.push({ label, command: rendered, log: logPath, exit: code });
  if (expectFailure) {
    if (code === 0) throw new Error(`${label} unexpectedly succeeded; see ${logPath}`);
    return `${stdout}\n${stderr}`;
  }
  if (code !== 0) {
    // The work directory is discarded with the runner, so the failure must carry its own output.
    const tail = `${stdout}\n${stderr}`.trim().split('\n').slice(-40).join('\n');
    throw new Error(`${label} failed with exit ${code}; see ${logPath}\n${tail}`);
  }
  return stdout;
}

function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

function option(name, fallback) {
  const index = process.argv.indexOf(name);
  return index === -1 ? fallback : process.argv[index + 1];
}

function required(name) {
  const value = option(name, undefined);
  if (!value) throw new Error(`missing ${name}`);
  return value;
}

// Loaded with `node --import` before the consumer: every socket, DNS, HTTP and
// fetch entry point records the attempt and throws, so a parse that needed the
// network fails instead of silently downloading.
const DENY_NETWORK = `import dns from 'node:dns';
import http from 'node:http';
import https from 'node:https';
import net from 'node:net';
import tls from 'node:tls';

globalThis.__issue195NetworkAttempts = [];
const deny = (name) => function denied(...args) {
  globalThis.__issue195NetworkAttempts.push(name);
  throw new Error(\`network access is disabled for the offline consumer: \${name}\`);
};
net.Socket.prototype.connect = deny('net.Socket.connect');
net.connect = deny('net.connect');
net.createConnection = deny('net.createConnection');
tls.connect = deny('tls.connect');
http.request = deny('http.request');
http.get = deny('http.get');
https.request = deny('https.request');
https.get = deny('https.get');
dns.lookup = deny('dns.lookup');
dns.resolve = deny('dns.resolve');
dns.promises.lookup = deny('dns.promises.lookup');
dns.promises.resolve = deny('dns.promises.resolve');
globalThis.fetch = deny('fetch');
`;

const NPM_CONSUMER = `import { readFileSync, writeFileSync } from 'node:fs';
import * as api from 'meta-language';

const [corpusPath, outputPath] = process.argv.slice(2);
const corpus = JSON.parse(readFileSync(corpusPath, 'utf8'));
const { LinkNetwork, LinkType, analyzeProgram, constructProgram, translateProgram, decodeProgramTranslation } = api;
let privatePathRejected = false;
try {
  await import(corpus.delivery.privateJavaScriptPath);
} catch (error) {
  privatePathRejected = error.code === 'ERR_PACKAGE_PATH_NOT_EXPORTED';
}
const compare = (left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right));
const parses = corpus.delivery.consumerCorpus.map(({ language, source, requiredTerm }) => {
  const network = LinkNetwork.parse(source, language);
  const rows = network.links()
    .map((link) => link.metadata())
    .filter((metadata) => metadata.linkType === LinkType.Syntax && metadata.span)
    .map(({ term, named, span, flags }) => \`\${term} \${named ? 1 : 0} \${span.byteRange.start}-\${span.byteRange.end} \${flags.isError ? 'E' : ''}\${flags.isMissing ? 'M' : ''}\${flags.isExtra ? 'X' : ''}\`)
    .sort(compare);
  return {
    language,
    clean: network.verifyFullMatch().isClean(),
    reconstructs: network.reconstructText() === source,
    requiredTermPresent: rows.some((row) => row.startsWith(\`\${requiredTerm} \`)),
    rows,
  };
});
const programs = corpus.delivery.programs.map(({ language, source }) => ({
  language,
  bindings: analyzeProgram(source, language).bindings.map(({ name, kind }) => \`\${name}:\${kind}\`).sort(compare),
  emitted: constructProgram(source, language).emit() === source,
}));
const translations = corpus.delivery.translations.map(({ source, sourceLanguage, targetLanguage }) => {
  const translated = translateProgram(source, sourceLanguage, targetLanguage);
  return {
    sourceLanguage,
    targetLanguage,
    code: translated.code,
    decodes: decodeProgramTranslation(translated.code, targetLanguage).source === source,
  };
});
const networkAttempts = [...(globalThis.__issue195NetworkAttempts ?? ['guard missing'])];
let guardRejectsNetwork = false;
try {
  await fetch('http://127.0.0.1:9/');
} catch (error) {
  guardRejectsNetwork = /network access is disabled/u.test(error.message);
}
writeFileSync(outputPath, JSON.stringify({
  publicEntryPoints: {
    used: corpus.delivery.publicEntryPoints.javascript.filter((name) => api[name] !== undefined),
    privatePathRejected,
  },
  offline: { networkAttempts, guardRejectsNetwork },
  observations: { parses, programs, translations },
}, null, 2));
`;

const CRATE_CONSUMER = `use meta_language::{
    analyze_program, construct_program, decode_program_translation, translate_program,
    LinkNetwork, LinkType, ParseConfiguration, ProgramProjectContext,
};
use serde_json::{json, Value};

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().expect("corpus string")
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let corpus: Value =
        serde_json::from_str(&std::fs::read_to_string(&arguments[1]).expect("corpus")).expect("json");
    let delivery = &corpus["delivery"];
    let parses: Vec<Value> = delivery["consumerCorpus"]
        .as_array()
        .expect("consumer corpus")
        .iter()
        .map(|case| {
            let source = text(case, "source");
            let network = LinkNetwork::parse(source, text(case, "language"), ParseConfiguration::default());
            let mut rows: Vec<String> = network
                .links()
                .map(|link| link.metadata())
                .filter(|metadata| metadata.link_type() == Some(LinkType::Syntax))
                .filter_map(|metadata| {
                    let span = metadata.span()?;
                    let range = span.byte_range();
                    let flags = metadata.flags();
                    Some(format!(
                        "{} {} {}-{} {}{}{}",
                        metadata.term().unwrap_or_default(),
                        u8::from(metadata.is_named()),
                        range.start(),
                        range.end(),
                        if flags.is_error() { "E" } else { "" },
                        if flags.is_missing() { "M" } else { "" },
                        if flags.is_extra() { "X" } else { "" },
                    ))
                })
                .collect();
            rows.sort();
            let required = format!("{} ", text(case, "requiredTerm"));
            json!({
                "language": text(case, "language"),
                "clean": network.verify_full_match(None).is_clean(),
                "reconstructs": network.reconstruct_text() == source,
                "requiredTermPresent": rows.iter().any(|row| row.starts_with(&required)),
                "rows": rows,
            })
        })
        .collect();
    let programs: Vec<Value> = delivery["programs"]
        .as_array()
        .expect("programs")
        .iter()
        .map(|case| {
            let source = text(case, "source");
            let language = text(case, "language");
            let program = analyze_program(source, language, ProgramProjectContext::default())
                .expect("analyze");
            let mut bindings: Vec<String> = program
                .bindings()
                .iter()
                .map(|binding| format!("{}:{}", binding.name(), binding.kind()))
                .collect();
            bindings.sort();
            let emitted = construct_program(source, language, ProgramProjectContext::default())
                .expect("construct")
                .emit();
            json!({ "language": language, "bindings": bindings, "emitted": emitted == source })
        })
        .collect();
    let translations: Vec<Value> = delivery["translations"]
        .as_array()
        .expect("translations")
        .iter()
        .map(|case| {
            let source = text(case, "source");
            let target = text(case, "targetLanguage");
            let translated =
                translate_program(source, text(case, "sourceLanguage"), target).expect("translate");
            let decoded = decode_program_translation(translated.code(), target).expect("decode");
            json!({
                "sourceLanguage": text(case, "sourceLanguage"),
                "targetLanguage": target,
                "code": translated.code(),
                "decodes": decoded.source() == source,
            })
        })
        .collect();
    let report = json!({
        "publicEntryPoints": {
            "used": delivery["publicEntryPoints"]["rust"],
        },
        "observations": { "parses": parses, "programs": programs, "translations": translations },
    });
    std::fs::write(&arguments[2], serde_json::to_string_pretty(&report).expect("report"))
        .expect("write report");
}
`;

await main();
