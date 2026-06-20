module.exports = grammar({
  name: "sum",
  rules: {
    sum: $ => seq($.num, repeat(seq("+", $.num))),
    num: $ => token(repeat1(/[0-9]/)),
  },
});
