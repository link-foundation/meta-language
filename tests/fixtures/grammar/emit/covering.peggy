start = &"pre" !"skip" ("hello"i / "hi") ("a" digit)? digit* digit+ "x"|2..4| "y"|3..| "z"|2| [\-a-f] [^q0-9] [A-Z] . digit label:"cap" "anon"
// A decimal digit.
digit = [0-9]
// NOTE: unordered choice in source is emitted as ordered Peggy choice.
unordered = "left" / "right"
// NOTE: Peggy has no atomic rule modifier; this rule returns its matched text.
atomic_rule = ("atomic") { return text(); }
// NOTE: Peggy has no silent rule modifier; this rule is emitted as a normal rule.
silent_rule = " "
// NOTE: Peggy has no token rule modifier; this rule returns its matched text.
token_rule = ("token") { return text(); }
empty = ""
