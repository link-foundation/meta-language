Preserve anonymous terminal extras and directly imported private scanner extras
as separators while retaining named aliases, scanner state and exact source
spans. Keep Pascal root spans at the input start through the existing leading
trivia projection policy.

Retain private scanner identities during lexing and recovery while exposing
anonymous tokens publicly, so Odin recovery ERROR spans keep their original
terminal newlines.
