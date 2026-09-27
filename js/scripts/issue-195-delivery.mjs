/**
 * Joins clean-consumer platform reports to the delivery assertions of issue #195.
 *
 * A report only counts for the candidate it names: the same corpus bytes, the same
 * package version and the exact candidate checksums.  Every rejected platform keeps
 * its reason so a missing assertion can be traced to the report that failed it.
 */

export const DELIVERY_CONSUMER_ASSERTIONS = [
  'cleanEnvironment',
  'exactArtifactChecksum',
  'publicEntryPoints',
  'offlineFirstParse',
];

/**
 * Evaluates one package side (`npm` or `crate`) of a platform report.
 *
 * `context` is `{ corpus, corpusSha256, version }`: the parsed evidence corpus, the
 * SHA-256 of its checked-in bytes and the candidate package version.
 */
export function deliveryChecks(report, side, expectedSha256, context) {
  const observed = report?.[side];
  if (!observed || report.corpusSha256 !== context.corpusSha256) return {};
  const { parses = [], programs = [], translations = [] } = observed.observations ?? {};
  const { delivery } = context.corpus;
  const corpusParsed = parses.length === delivery.consumerCorpus.length &&
    parses.every(({ clean, reconstructs, requiredTermPresent }) =>
      clean && reconstructs && requiredTermPresent) &&
    programs.length === delivery.programs.length &&
    programs.every(({ emitted, bindings }) => emitted && bindings.length > 0) &&
    translations.length === delivery.translations.length &&
    translations.every(({ decodes }) => decodes);
  const offline = side === 'npm'
    ? observed.offline?.networkAttempts?.length === 0 && observed.offline.guardRejectsNetwork === true
    : observed.offline?.builtOffline === true &&
      observed.offline.ranWithNetworkDisabledEnvironment === true &&
      observed.offline.networkDependencies?.length === 0 &&
      observed.offline.networkApiFiles?.length === 0;
  const clean = Object.values(observed.clean ?? {});
  const declared = delivery.publicEntryPoints[side === 'npm' ? 'javascript' : 'rust'];
  return {
    cleanEnvironment: clean.length > 0 && clean.every((value) => value === true),
    exactArtifactChecksum: observed.version === context.version &&
      observed.checksum?.sha256 === expectedSha256 &&
      observed.checksum.expectedSha256 === expectedSha256 &&
      (side !== 'npm' || observed.checksum.installedIntegrity === observed.checksum.expectedIntegrity),
    publicEntryPoints: declared.every((name) => observed.publicEntryPoints?.used?.includes(name)) &&
      observed.publicEntryPoints.privatePathRejected === true,
    offlineFirstParse: offline && corpusParsed,
  };
}

/**
 * The consumer assertions one report satisfies for the given package sides; a
 * cross-runtime cell also needs the npm and crate observations to agree.
 */
export function deliveryAssertions(report, sides, expected, context) {
  const checks = sides.map((side) => deliveryChecks(report, side, expected[side], context));
  const agrees = sides.length === 1 || report?.agreement?.mismatches?.length === 0;
  return Object.fromEntries(DELIVERY_CONSUMER_ASSERTIONS.map((assertion) => [
    assertion,
    agrees && checks.every((check) => check[assertion] === true),
  ]));
}

/**
 * Whether every supported platform produced at least one report and every report
 * for it satisfies all consumer assertions, with a reason for each platform that
 * did not.
 */
export function supportedPlatformCoverage(reports, sides, expected, context) {
  const failures = [];
  for (const platform of context.corpus.delivery.supportedPlatforms) {
    const platformReports = reports.filter((report) => report?.platform === platform);
    if (platformReports.length === 0) {
      failures.push({ platform, reason: 'no consumer report' });
      continue;
    }
    for (const report of platformReports) {
      const reason = rejection(report, sides, expected, context);
      if (reason) failures.push({ platform, reason });
    }
  }
  return { observed: failures.length === 0, failures };
}

function rejection(report, sides, expected, context) {
  if (report.corpusSha256 !== context.corpusSha256) {
    return `corpus SHA-256 ${report.corpusSha256} differs from the checked-in ${context.corpusSha256}`;
  }
  const missing = sides.filter((side) => !report[side]);
  if (missing.length > 0) return `no ${missing.join(' and ')} consumer observations`;
  if (sides.length > 1 && report.agreement?.mismatches?.length !== 0) {
    return `npm and crate observations disagree: ${JSON.stringify(report.agreement?.mismatches ?? null)}`;
  }
  const assertions = deliveryAssertions(report, sides, expected, context);
  const failed = Object.keys(assertions).filter((assertion) => !assertions[assertion]);
  return failed.length > 0 ? `failed ${failed.join(', ')}` : null;
}
