import {versionParts} from './semver-range.mjs';

/** Preserve explicit minimum coverage and every supplied release coordinate. */
export function readNodeRuntimeComparison(pin,isFloor) {
  if(isFloor)return 'floor';
  const parts=versionParts(pin);
  if(parts?.[2]!=null)return 'version';
  if(parts?.[1]!=null)return 'minor';
  return 'major';
}
