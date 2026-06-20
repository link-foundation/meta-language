bump: minor

- Add a grammar-backed `TypeScript` front end using `tree-sitter-typescript`,
  wiring the `typescript`/`ts` labels to `LANGUAGE_TYPESCRIPT` and the `tsx`
  label to `LANGUAGE_TSX` so `LinkNetwork::parse` emits real `LinkType::Syntax`
  links for TypeScript and TSX sources.
