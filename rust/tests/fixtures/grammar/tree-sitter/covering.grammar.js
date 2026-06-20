module.exports = grammar({
  name: "start",
  inline: $ => [
    $._silent_rule,
  ],
  rules: {
    start: $ => seq("start", /[Cc][Aa][Ss][Ee]/, choice("a", "b"), optional(seq("x", $.digit)), repeat($.digit), repeat1($.digit), seq("m", "m", optional("m"), optional("m")), seq("n", "n", "n", repeat("n")), blank(), /[\-a-f\/\]]/, /[^q0-9]/, /[A-Z]/, /./, $.digit, field("label", "cap"), "anon"),
    // A decimal digit.
    digit: $ => /[0-9]/,
    ordered: $ => choice("left", "right"),
    atomic_rule: $ => token("atomic"),
    _silent_rule: $ => " ",
    token_rule: $ => token("token"),
    empty: $ => blank(),
  },
});
