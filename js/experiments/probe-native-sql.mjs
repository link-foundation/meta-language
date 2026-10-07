// Own minimized cases against the pinned SQL oracle; no upstream corpus locally.
process.argv[2] = 'sql';
await import('./probe-native-grammar-contexts.mjs');
