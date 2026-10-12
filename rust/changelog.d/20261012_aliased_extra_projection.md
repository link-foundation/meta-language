Aliased extras retain their source lexer and scanner identity, then expose the
declared alias in the concrete tree. Shared fixtures retain scanner state,
skipped whitespace, Unicode byte spans and captured fields; anonymous trivia
remains distinct from significant anonymous syntax during oracle projection.
