Use Rust's exact Pattern_White_Space set when the JavaScript frontend tokenizes
Rust source. Preserve U+0085, U+200E and U+200F as valid separators, and refuse
non-Rust spaces such as NBSP and noninitial byte order marks. Token spans retain
offsets in the original source, including an initial byte order mark. Other
languages retain their existing whitespace interpretation.
