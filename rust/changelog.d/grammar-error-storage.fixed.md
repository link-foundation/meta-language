Store `GrammarParseError.rejection` as `Box<ParseRejection>` to keep returned
grammar errors small as diagnostics grow. The outer parse result and error types,
diagnostic contents and error messages remain unchanged; callers constructing the
public error field directly must box its rejection value.
