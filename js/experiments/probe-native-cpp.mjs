// Own minimized cases against the pinned C++ oracle; no upstream corpus locally.
process.argv[2] = 'cpp';
await import('./probe-native-grammar-contexts.mjs');
