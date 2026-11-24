// Authored compiler diagnostic obligations, not execution observations. The
// normal policy generator publishes these as Links after the full draft gate.
export function rustSourceStateAssertionPolicies(){
 const cases=[['valid-consumer',[]],['pointer-width-rejection',['TS2322']],['exclusive-profile-rejection',['TS2322']],['resource-handler-rejection',['TS2741']],['concrete-identity-rejection',['TS2322']]];
 return cases.map(([name,expected])=>({id:'typescript-rust-state-'+name+'-diagnostic',subject:'TypeScript public declaration compiler',comparison:{kind:'release',precision:3},expectations:['5.9.2','7.0.2'].map(version=>({id:'typescript-'+version.replaceAll('.','-')+'-rust-state-'+name,minimum:version,maximum:version,includeMinimum:true,includeMaximum:true,expected,evidence:'Authored public Rust source state consumer obligation for explicitly selected TypeScript '+version+'. The original consumer input and exact diagnostic inventory remain required; no execution observation is claimed.'}))}));
}
