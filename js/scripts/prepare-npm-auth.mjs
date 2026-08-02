#!/usr/bin/env node
// Prepare npm authentication for the publish job.
//
// Root cause this script exists for (issue #191):
// `actions/setup-node` with `registry-url` unconditionally writes
// `//registry.npmjs.org/:_authToken=${NODE_AUTH_TOKEN}` into the npm user
// config and exports the placeholder `XXXXX-XXXXX-XXXXX-XXXXX` when no token
// was supplied. npm then believes credentials are already configured, never
// performs the OIDC token exchange, and the registry rejects the anonymous
// PUT with a misleading `E404 Not Found - PUT .../meta-language`.
// See https://github.com/actions/setup-node/issues/1551.
//
// Removing the placeholder line makes npm fall through to the trusted
// publishing (OIDC) path. When a real NPM_TOKEN is supplied the line is kept
// so the classic token flow still works as a bootstrap fallback.

import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

// The literal value actions/setup-node exports when NODE_AUTH_TOKEN is unset.
export const SETUP_NODE_PLACEHOLDER_TOKEN = 'XXXXX-XXXXX-XXXXX-XXXXX';

const AUTH_TOKEN_LINE = /^[^\S\r\n]*(?:\/\/[^\s=]*:)?_authToken[^\S\r\n]*=.*(?:\r?\n|$)/gim;
const ALWAYS_AUTH_LINE = /^[^\S\r\n]*always-auth[^\S\r\n]*=.*(?:\r?\n|$)/gim;

/**
 * A token is usable only when it is a real credential. The setup-node
 * placeholder and an unexpanded `${NODE_AUTH_TOKEN}` are both unusable.
 * @param {string|undefined} token
 * @returns {boolean}
 */
export function isUsableToken(token) {
  const value = String(token ?? '').trim();
  if (value === '') return false;
  if (value === SETUP_NODE_PLACEHOLDER_TOKEN) return false;
  if (/^\$\{?NODE_AUTH_TOKEN\}?$/.test(value)) return false;
  return true;
}

/**
 * Decide which credential npm should use.
 * @param {Record<string, string|undefined>} env
 * @returns {'token'|'oidc'|'none'}
 */
export function resolveAuthMode(env = process.env) {
  if (isUsableToken(env.NPM_TOKEN) || isUsableToken(env.NODE_AUTH_TOKEN)) {
    return 'token';
  }
  if (env.ACTIONS_ID_TOKEN_REQUEST_URL && env.ACTIONS_ID_TOKEN_REQUEST_TOKEN) {
    return 'oidc';
  }
  return 'none';
}

/**
 * Strip the entries that block the OIDC handshake from an npmrc body.
 * `always-auth` is also removed because npm 11 warns about the deprecated key.
 * @param {string} content
 * @param {{ keepAuthToken?: boolean }} [options]
 * @returns {{content: string, removedAuthToken: boolean, removedAlwaysAuth: boolean}}
 */
export function sanitizeNpmrc(content, { keepAuthToken = false } = {}) {
  const original = String(content);
  const withoutAuthToken = keepAuthToken ? original : original.replace(AUTH_TOKEN_LINE, '');
  const sanitized = withoutAuthToken.replace(ALWAYS_AUTH_LINE, '');
  return {
    content: sanitized,
    removedAuthToken: withoutAuthToken !== original,
    removedAlwaysAuth: sanitized !== withoutAuthToken,
  };
}

/**
 * Guidance printed when no credential is available at all, so the failure is
 * actionable instead of surfacing as a bare `E404`.
 * @param {string} packageName
 * @param {string} repository
 * @param {string} workflow
 * @returns {string}
 */
export function buildMissingCredentialGuidance(packageName, repository, workflow) {
  return [
    `No npm credential is available to publish ${packageName}.`,
    'Configure exactly one of:',
    `  1. OIDC trusted publishing (preferred): on npmjs.com open the ${packageName} package`,
    `     settings, add a trusted publisher for repository ${repository} with workflow`,
    `     file ${workflow}, and keep "permissions: id-token: write" on the publish job.`,
    '  2. A classic automation token: add an NPM_TOKEN repository secret. This is the',
    '     bootstrap path for a package that has no trusted publisher yet; it can be removed',
    '     once trusted publishing is configured.',
  ].join('\n');
}

/**
 * Rewrite the npm user config so the resolved auth mode can actually be used.
 * @param {object} [options]
 * @param {Record<string, string|undefined>} [options.env]
 * @param {{log: Function, warn: Function}} [options.logger]
 * @param {boolean} [options.verbose]
 * @param {Function} [options.fileExists]
 * @param {Function} [options.readFile]
 * @param {Function} [options.writeFile]
 * @returns {{mode: string, path: string, changed: boolean, skipped: boolean}}
 */
export function prepareNpmAuth({
  env = process.env,
  logger = console,
  verbose = false,
  fileExists = existsSync,
  readFile = readFileSync,
  writeFile = writeFileSync,
} = {}) {
  const mode = resolveAuthMode(env);
  const userConfigPath =
    env.NPM_CONFIG_USERCONFIG || (env.HOME ? path.join(env.HOME, '.npmrc') : '');

  logger.log(`npm auth mode: ${mode}`);

  if (!userConfigPath) {
    logger.warn('No npm user config path resolved; nothing to sanitize.');
    return { mode, path: '', changed: false, skipped: true };
  }

  if (!fileExists(userConfigPath)) {
    logger.warn(`npm user config does not exist: ${userConfigPath}`);
    return { mode, path: userConfigPath, changed: false, skipped: true };
  }

  const original = readFile(userConfigPath, 'utf8');
  const result = sanitizeNpmrc(original, { keepAuthToken: mode === 'token' });

  if (verbose) {
    // Only key names are printed; values are never logged so a real token
    // cannot leak into the workflow log.
    const keys = original
      .split(/\r?\n/)
      .filter((line) => line.trim() !== '' && !line.trim().startsWith(';'))
      .map((line) => line.split('=')[0].trim());
    logger.log(`npm user config: ${userConfigPath}`);
    logger.log(`npm user config keys before sanitize: ${keys.join(', ') || '(none)'}`);
  }

  const changed = result.removedAuthToken || result.removedAlwaysAuth;
  if (!changed) {
    logger.log(`No blocking npm auth entries found in ${userConfigPath}.`);
    return { mode, path: userConfigPath, changed: false, skipped: false };
  }

  writeFile(userConfigPath, result.content);
  if (result.removedAuthToken) {
    logger.log(
      `Removed the placeholder _authToken entry from ${userConfigPath} so npm can perform the OIDC exchange (actions/setup-node#1551).`,
    );
  }
  if (result.removedAlwaysAuth) {
    logger.log(`Removed the deprecated always-auth entry from ${userConfigPath}.`);
  }
  return { mode, path: userConfigPath, changed: true, skipped: false };
}

function isMainModule() {
  return process.argv[1]
    ? path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
    : false;
}

if (isMainModule()) {
  // Verbose diagnostics are off by default; enable with --verbose or
  // NPM_AUTH_DEBUG=1 when a publish failure needs to be traced.
  const verbose =
    process.argv.includes('--verbose') || process.env.NPM_AUTH_DEBUG === '1';
  const { mode } = prepareNpmAuth({ verbose });

  if (process.env.GITHUB_OUTPUT) {
    writeFileSync(process.env.GITHUB_OUTPUT, `auth_mode=${mode}\n`, { flag: 'a' });
  }

  if (mode === 'none') {
    const guidance = buildMissingCredentialGuidance(
      process.env.PACKAGE_NAME || 'meta-language',
      process.env.GITHUB_REPOSITORY || 'link-foundation/meta-language',
      process.env.PUBLISH_WORKFLOW_FILE || '.github/workflows/js.yml',
    );
    console.error(`::error::${guidance.split('\n')[0]}`);
    console.error(guidance);
    process.exit(1);
  }
}
