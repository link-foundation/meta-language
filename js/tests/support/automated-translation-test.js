export const automatedTranslationTest = Object.freeze({
  skip: process.env.META_LANGUAGE_AUTOMATED_TRANSLATION === 'false' && 'Automated translation is paused',
});
