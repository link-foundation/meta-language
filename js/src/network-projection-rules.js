// Projection membership only. The source network retains every original
// link, reference, span and recovery flag; these decisions never rewrite it.

/** @param {string} projection @returns {boolean} */
export function acceptNetworkProjection(projection) {
  return projection === 'Lossless' || projection === 'ConcreteSyntax'
    || projection === 'AbstractSyntax' || projection === 'Semantic';
}

/** @param {string} projection @param {string} linkType @returns {boolean} */
export function acceptNetworkProjectionLink(projection, linkType) {
  if (projection === 'Lossless') return true;
  if (projection === 'ConcreteSyntax') return linkType !== 'Semantic';
  if (projection === 'AbstractSyntax') return linkType !== 'SourceToken' && linkType !== 'Trivia';
  if (projection === 'Semantic') {
    return linkType === 'Semantic' || linkType === 'Concept'
      || linkType === 'Type' || linkType === 'Language';
  }
  return false;
}
