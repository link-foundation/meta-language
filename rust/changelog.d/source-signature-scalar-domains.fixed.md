Reject numeric-string widths and non-Boolean signedness in owned Rust source
signature types before JavaScript type-key comparison, including nested tuple
and array positions. The existing generated integral Number policy retains valid
width aliases. This closes a mismatch with native typed signature decoding.
