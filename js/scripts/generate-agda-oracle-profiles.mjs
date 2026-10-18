// Bounded expected-data derivation only; never builds or executes an oracle.
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { AGDA_CURRENT_ORACLE_PROFILE_FILE, readAgdaCurrentOracleFixture } from './agda-oracle-profiles.mjs';

export function generateAgdaOracleProfiles(check = false) {
  const file = new URL(`../../${AGDA_CURRENT_ORACLE_PROFILE_FILE}`, import.meta.url);
  const expected = readAgdaCurrentOracleFixture();
  if (check) checkProjectData(file, expected);
  else writeProjectData(file, expected);
  return expected;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  generateAgdaOracleProfiles(process.argv.includes('--check'));
  console.log('Agda current oracle profile: all original cases retained; corrected clean-row expectations authored, execution remains a CI check');
}
