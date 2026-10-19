JavaScript translation checks require an explicit native Rust opt-in, with
`TRANSLATION_CHECK_RUST=0` retaining JavaScript-only execution. Acceptance
observation stages still require native compilation and behavior checks.
The source, translation, and JavaScript behavior assertions remain active
when native compilation is disabled.
