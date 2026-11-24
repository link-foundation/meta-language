// Private control flow for a checked source abort. It carries no value, region
// or state certificate. Only the actual abort expression raises this signal.
class RustSourceRegionDivergence extends Error {}
export function divergeRustSourceRegions(){throw new RustSourceRegionDivergence();}
export function isRustSourceRegionDivergence(error){return error instanceof RustSourceRegionDivergence;}
