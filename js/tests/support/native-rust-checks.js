// Native checks run only with an explicit opt-in, or in the acceptance stage
// whose observation file requires the installed native toolchains.
export function nativeRustChecksEnabled(environment = process.env, flag = 'TRANSLATION_CHECK_RUST') {
  return environment[flag] === '1' || Boolean(environment.ISSUE_195_OBSERVATION_FILE);
}
