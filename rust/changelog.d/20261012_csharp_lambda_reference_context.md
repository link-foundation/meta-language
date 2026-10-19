Recorded lambda parameter contexts preserve the pinned C# plain `ref` type tree
while retaining `ref readonly`, `scoped ref` and attributed parameter modifier
aliases. Separate exact CI, comment and attribute fixtures check those preferences
alongside the previous five declaration regressions at the default parser limits.
