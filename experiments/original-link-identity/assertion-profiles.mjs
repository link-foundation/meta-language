// Additive test assertion profiles. The original executable policy and all
// source pins stay frozen; current diagnostics never rewrite that baseline.
import {originalLinkIdentityPolicy} from './policy.mjs';

export const originalLinkIdentityAssertionProfiles={
 schemaVersion:1,model:'profile-qualified original link identity feasibility assertions 1',
 frozenSourceInputs:[{"path":"policy.mjs","sha256":"080d379c8c97696fb611f841b1d7dba220864e4cd82984febb5c550e7bdd46e5","sourceBytes":8168},{"path":"data/policy.lino","sha256":"c287a0ef8c8eba5a8ee077924fc0f05b306c8e03e44ad0033d4c6f437eb4ed89","sourceBytes":24703}],
 originalPolicy:originalLinkIdentityPolicy,
 publishedGrowthMetadata:{"commit":"dfe0da8d8d836ec2dbae9d29d6e054b079bdf30e","path":"experiments/original-link-identity/policy.mjs","sourceSha256":"7ba1588e8e978ccb5e2b97f6b6f06b7e71b5d003701b80d9b08752dc25288c03","sourceBytes":9161,"growthMetadataSource":"  currentGrowth:{kind:'type',reason:'expected float but found option:undefined:float',span:{start:165,end:177},\n   observation:'CI 38911564946 source-models job 117015296609 retained the exact identityGrowth source and reported this earlier argument-type refusal after fresh-local append promotion.',\n   boundary:'The original unguarded words[index] read remains optional. Fresh append admission does not erase an absent element or prove this indexed loop read total.'},\n  growthProfiles:[\n   {profile:'automatic',expected:'currentGrowth',provenance:'The original CI diagnostic remains the active automatic-owned construction refusal.'},\n   {profile:'owned',expected:'currentGrowth',provenance:'Explicit referenceSemantics:false selects the same owned construction; the optional read remains unproved.'},\n   {profile:'reference',expected:'growth',provenance:'B46 retains explicit reference operations instead of privately rebinding them. The original whole-push refusal remains exact.'},\n  ],\n","growthMetadataSha256":"3bd7cc0d2b5e1335afc7809d6704b3cff91edc0a00d10c9045a0e9650df3525a"},
 feasibility:{
  growth:originalLinkIdentityPolicy.feasibility.growth,
  arrayFrom:originalLinkIdentityPolicy.feasibility.arrayFrom,
  currentGrowth:{kind:'type',reason:'expected float but found option:undefined:float',span:{start:165,end:177},
   observation:'CI 38911564946 source-models job 117015296609 retained the exact identityGrowth source and reported this earlier argument-type refusal after fresh-local append promotion.',
   boundary:'The original unguarded words[index] read remains optional. Fresh append admission does not erase an absent element or prove this indexed loop read total.'},
  growthProfiles:[
   {profile:'automatic',expected:'currentGrowth',provenance:'The original CI diagnostic remains the active automatic-owned construction refusal.'},
   {profile:'owned',expected:'currentGrowth',provenance:'Explicit referenceSemantics:false selects the same owned construction; the optional read remains unproved.'},
   {profile:'reference',expected:'growth',provenance:'B46 retains explicit reference operations instead of privately rebinding them. The original whole-push refusal remains exact.'},
  ],
 },
 boundary:'These assertion profiles preserve the published historical controls and CI diagnostic provenance. They supplement the unchanged original policy; they do not change the native kernel, original source receipts, default scope or native observations.',
};
