const ml_big_integer_conversion = (() => {
// Translated from JavaScript by meta-language: portable core, JavaScript target.
'use strict';

// Integer division with the source's rounding; by zero it aborts with the message `zero`
// or, when that is null, is total (x / 0 = 0, x % 0 = x); a machine-integer quotient
// out of [min, max] aborts with `overflow`, the remainder too.
function ml_divide(a, b, rounding, zero, remainder, bounds) {
  if (b === 0n) {
    if (zero !== null) throw new RangeError(zero);
    return remainder ? a : 0n;
  }
  let q = a / b;
  const r = a - q * b;
  if (r !== 0n) {
    if (rounding === 'floor' && (r < 0n) !== (b < 0n)) q -= 1n;
    if (rounding === 'euclid' && r < 0n) q = b > 0n ? q - 1n : q + 1n;
  }
  if (bounds && (q < bounds[0] || q > bounds[1])) throw new RangeError(bounds[2]);
  return remainder ? a - q * b : q;
}

function acceptJavaScriptWhitespaceUnit(unit) {
  return ((((((((((((unit >= 9) && (unit <= 13)) || (unit === 32)) || (unit === 160)) || (unit === 5760)) || ((unit >= 8192) && (unit <= 8202))) || (unit === 8232)) || (unit === 8233)) || (unit === 8239)) || (unit === 8287)) || (unit === 12288)) || (unit === 65279));
}

function readJavaScriptNumericDigitUnit(unit) {
  if (((unit >= 48) && (unit <= 57))) {
    return (unit - 48);
  } else {
    if (((unit >= 65) && (unit <= 70))) {
      return (unit - 55);
    } else {
      if (((unit >= 97) && (unit <= 102))) {
        return (unit - 87);
      } else {
        return (-1);
      }
    }
  }
}

function readJavaScriptUtf8StringUnits(bytes) {
  const index = 0;
  const source_ml_source2_s1 = source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1(bytes, index);
  switch (source_ml_source2_s1.$) {
    case "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_done": {
      const units = [];
      const index_2 = 0;
      const source_ml_source2_s2 = source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2(bytes, units, index_2);
      switch (source_ml_source2_s2.$) {
        case "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_done": {
          const units_2 = source_ml_source2_s2["units"];
          return units_2;
        }
        case "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return": {
          const source_ml_source1_temporary_result2 = source_ml_source2_s2["ml_source1_temporary_result2"];
          return source_ml_source1_temporary_result2;
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source2_s2.$}`);
        }
      }
    }
    case "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_return": {
      const source_ml_source1_temporary_result4 = source_ml_source2_s1["ml_source1_temporary_result4"];
      return source_ml_source1_temporary_result4;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source2_s1.$}`);
    }
  }
}

function readStringNumberDigit(unit) {
  return readJavaScriptNumericDigitUnit(unit);
}

function readStringNumberDecimalPower(exponent) {
  const value = 1n;
  const base = 10n;
  const value_2 = source_ml_source1_temporary_readStringNumberDecimalPower_loop5(exponent, value, base);
  return value_2;
}

function readStringNumberExactMantissa(value) {
  const result = 0;
  const scale = 1;
  const result_2 = source_ml_source1_temporary_readStringNumberExactMantissa_loop6(value, result, scale);
  return result_2;
}

function readStringNumberBinaryPower(exponent) {
  const result = 1;
  const source_ml_source2_s3 = source_ml_source1_temporary_readStringNumberBinaryPower_loop7(exponent, result);
  switch (source_ml_source2_s3.$) {
    case "ml_source1_temporary_readStringNumberBinaryPower_loop7_done": {
      const exponent_2 = source_ml_source2_s3["exponent"];
      const result_2 = source_ml_source2_s3["result"];
      const result_3 = source_ml_source1_temporary_readStringNumberBinaryPower_loop8(exponent_2, result_2);
      return result_3;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source2_s3.$}`);
    }
  }
}

function convertStringNumberRational(numerator, denominator) {
  if (((numerator < 0n) || (denominator <= 0n))) {
    return NaN;
  } else {
    if ((numerator === 0n)) {
      return 0;
    } else {
      const exponent = 0;
      const source_ml_source2_s6 = ((numerator >= denominator) ? (() => {
        const scale = denominator;
        const source_ml_source2_s7 = source_ml_source1_temporary_convertStringNumberRational_loop15(numerator, exponent, scale);
        switch (source_ml_source2_s7.$) {
          case "ml_source1_temporary_convertStringNumberRational_loop15_done": {
            const exponent_2 = source_ml_source2_s7["exponent"];
            return ({ $: "ml_source1_temporary_convertStringNumberRational_join9_next", ["exponent"]: exponent_2 });
          }
          case "ml_source1_temporary_convertStringNumberRational_loop15_return": {
            const source_ml_source1_temporary_result15 = source_ml_source2_s7["ml_source1_temporary_result15"];
            return ({ $: "ml_source1_temporary_convertStringNumberRational_join9_return", ["ml_source1_temporary_result14"]: source_ml_source1_temporary_result15 });
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source2_s7.$}`);
          }
        }
      })() : (() => {
        const scale_2 = numerator;
        const source_ml_source2_s8 = source_ml_source1_temporary_convertStringNumberRational_loop16(denominator, exponent, scale_2);
        switch (source_ml_source2_s8.$) {
          case "ml_source1_temporary_convertStringNumberRational_loop16_done": {
            const exponent_3 = source_ml_source2_s8["exponent"];
            return ({ $: "ml_source1_temporary_convertStringNumberRational_join9_next", ["exponent"]: exponent_3 });
          }
          case "ml_source1_temporary_convertStringNumberRational_loop16_return": {
            const source_ml_source1_temporary_result16 = source_ml_source2_s8["ml_source1_temporary_result16"];
            return ({ $: "ml_source1_temporary_convertStringNumberRational_join9_return", ["ml_source1_temporary_result14"]: source_ml_source1_temporary_result16 });
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source2_s8.$}`);
          }
        }
      })());
      switch (source_ml_source2_s6.$) {
        case "ml_source1_temporary_convertStringNumberRational_join9_next": {
          const exponent_4 = source_ml_source2_s6["exponent"];
          const shift = ((exponent_4 >= (-1022)) ? (52 - exponent_4) : 1074);
          const source_ml_source2_s9 = ((shift >= 0) ? (() => {
            const index = 0;
            const numerator_2 = source_ml_source1_temporary_convertStringNumberRational_loop13(numerator, shift, index);
            return ({ $: "ml_source1_temporary_convertStringNumberRational_join10_next", ["numerator"]: numerator_2, ["denominator"]: denominator });
          })() : (() => {
            const index_2 = 0;
            const denominator_2 = source_ml_source1_temporary_convertStringNumberRational_loop14(denominator, shift, index_2);
            return ({ $: "ml_source1_temporary_convertStringNumberRational_join10_next", ["numerator"]: numerator, ["denominator"]: denominator_2 });
          })());
          switch (source_ml_source2_s9.$) {
            case "ml_source1_temporary_convertStringNumberRational_join10_next": {
              const numerator_3 = source_ml_source2_s9["numerator"];
              const denominator_3 = source_ml_source2_s9["denominator"];
              const quotient = 0n;
              const remainder = numerator_3;
              const scale_3 = denominator_3;
              const weight = 1n;
              const source_ml_source2_s10 = source_ml_source1_temporary_convertStringNumberRational_loop11(remainder, scale_3, weight);
              switch (source_ml_source2_s10.$) {
                case "ml_source1_temporary_convertStringNumberRational_loop11_done": {
                  const scale_4 = source_ml_source2_s10["scale"];
                  const weight_2 = source_ml_source2_s10["weight"];
                  const source_ml_source2_s11 = source_ml_source1_temporary_convertStringNumberRational_loop12(quotient, remainder, scale_4, weight_2);
                  switch (source_ml_source2_s11.$) {
                    case "ml_source1_temporary_convertStringNumberRational_loop12_done": {
                      const quotient_2 = source_ml_source2_s11["quotient"];
                      const remainder_2 = source_ml_source2_s11["remainder"];
                      const twice = (remainder_2 * 2n);
                      const quotient_4 = (((twice > denominator_3) || ((twice === denominator_3) && (ml_divide(quotient_2, 2n, 'trunc', "Division by zero", true) === 1n))) ? (() => {
                        const quotient_3 = (quotient_2 + 1n);
                        return quotient_3;
                      })() : quotient_2);
                      return (readStringNumberExactMantissa(quotient_4) * readStringNumberBinaryPower(-shift));
                    }
                    default: {
                      throw new TypeError(`unexpected constructor ${source_ml_source2_s11.$}`);
                    }
                  }
                }
                default: {
                  throw new TypeError(`unexpected constructor ${source_ml_source2_s10.$}`);
                }
              }
            }
            default: {
              throw new TypeError(`unexpected constructor ${source_ml_source2_s9.$}`);
            }
          }
        }
        case "ml_source1_temporary_convertStringNumberRational_join9_return": {
          const source_ml_source1_temporary_result14 = source_ml_source2_s6["ml_source1_temporary_result14"];
          return source_ml_source1_temporary_result14;
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source2_s6.$}`);
        }
      }
    }
  }
}

function acceptStringNumberInfinity(units, start, end) {
  if (((end - start) !== 8)) {
    return false;
  } else {
    const expected = [73, 110, 102, 105, 110, 105, 116, 121];
    const index = 0;
    const source_ml_source2_s12 = source_ml_source1_temporary_acceptStringNumberInfinity_loop17(units, start, expected, index);
    switch (source_ml_source2_s12.$) {
      case "ml_source1_temporary_acceptStringNumberInfinity_loop17_done": {
        return true;
      }
      case "ml_source1_temporary_acceptStringNumberInfinity_loop17_return": {
        const source_ml_source1_temporary_result17 = source_ml_source2_s12["ml_source1_temporary_result17"];
        return source_ml_source1_temporary_result17;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source2_s12.$}`);
      }
    }
  }
}

function convertJavaScriptStringNumberUnits(units) {
  const index = 0;
  const source_ml_source2_s13 = source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18(units, index);
  switch (source_ml_source2_s13.$) {
    case "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_done": {
      const start = 0;
      const end = units.length;
      const start_2 = source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19(units, start, end);
      const end_2 = source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop20(units, start_2, end);
      if ((start_2 === end_2)) {
        return 0;
      } else {
        const negative = false;
        const signed = false;
        const sign = ((units)[start_2] ?? (-1));
        const source_ml_source2_s14 = (((sign === 43) || (sign === 45)) ? (() => {
          const negative_2 = (sign === 45);
          const signed_2 = true;
          const start_3 = (start_2 + 1);
          return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join21_next", ["start"]: start_3, ["negative"]: negative_2, ["signed"]: signed_2 });
        })() : ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join21_next", ["start"]: start_2, ["negative"]: negative, ["signed"]: signed }));
        switch (source_ml_source2_s14.$) {
          case "ml_source1_temporary_convertJavaScriptStringNumberUnits_join21_next": {
            const start_4 = source_ml_source2_s14["start"];
            const negative_3 = source_ml_source2_s14["negative"];
            const signed_3 = source_ml_source2_s14["signed"];
            if ((start_4 === end_2)) {
              return NaN;
            } else {
              if (acceptStringNumberInfinity(units, start_4, end_2)) {
                if (negative_3) {
                  return (-Infinity);
                } else {
                  return Infinity;
                }
              } else {
                const integerDigits = [0n, 1n, 2n, 3n, 4n, 5n, 6n, 7n, 8n, 9n, 10n, 11n, 12n, 13n, 14n, 15n];
                const source_ml_source2_s16 = (((!signed_3 && (((units)[start_4] ?? (-1)) === 48)) && ((end_2 - start_4) >= 2)) ? (() => {
                  const marker = ((units)[(start_4 + 1)] ?? (-1));
                  const radix = (((marker === 120) || (marker === 88)) ? 16 : (((marker === 98) || (marker === 66)) ? 2 : (((marker === 111) || (marker === 79)) ? 8 : 0)));
                  if ((radix !== 0)) {
                    const value = 0n;
                    const overflow = false;
                    const limit = readStringNumberDecimalPower(310);
                    const base = ((radix === 16) ? 16n : ((radix === 8) ? 8n : 2n));
                    const start_5 = (start_4 + 2);
                    if ((start_5 === end_2)) {
                      return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return", ["ml_source1_temporary_result28"]: NaN });
                    } else {
                      const index_2 = start_5;
                      const source_ml_source2_s17 = source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29(units, end_2, integerDigits, radix, value, overflow, limit, base, index_2);
                      switch (source_ml_source2_s17.$) {
                        case "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_done": {
                          const value_2 = source_ml_source2_s17["value"];
                          const overflow_2 = source_ml_source2_s17["overflow"];
                          return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return", ["ml_source1_temporary_result28"]: (overflow_2 ? Infinity : convertStringNumberRational(value_2, 1n)) });
                        }
                        case "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_return": {
                          const source_ml_source1_temporary_result29 = source_ml_source2_s17["ml_source1_temporary_result29"];
                          return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return", ["ml_source1_temporary_result28"]: source_ml_source1_temporary_result29 });
                        }
                        default: {
                          throw new TypeError(`unexpected constructor ${source_ml_source2_s17.$}`);
                        }
                      }
                    }
                  } else {
                    return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_next", ["start"]: start_4 });
                  }
                })() : ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_next", ["start"]: start_4 }));
                switch (source_ml_source2_s16.$) {
                  case "ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_next": {
                    const start_6 = source_ml_source2_s16["start"];
                    const at = start_6;
                    const fraction = 0;
                    const digits = 0;
                    const significant = 0;
                    const dot = false;
                    const source_ml_source2_s18 = source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23(units, end_2, at, fraction, digits, significant, dot);
                    switch (source_ml_source2_s18.$) {
                      case "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_done": {
                        const at_2 = source_ml_source2_s18["at"];
                        const fraction_2 = source_ml_source2_s18["fraction"];
                        const digits_2 = source_ml_source2_s18["digits"];
                        const significant_2 = source_ml_source2_s18["significant"];
                        if ((digits_2 === 0)) {
                          return NaN;
                        } else {
                          const decimalEnd = at_2;
                          const exponent = 0;
                          const exponentLimit = (units.length + 10000);
                          const source_ml_source2_s21 = (((at_2 < end_2) && ((((units)[at_2] ?? (-1)) === 101) || (((units)[at_2] ?? (-1)) === 69))) ? (() => {
                            const at_3 = (at_2 + 1);
                            const exponentNegative = false;
                            const exponentSign = ((units)[at_3] ?? (-1));
                            const source_ml_source2_s22 = (((exponentSign === 43) || (exponentSign === 45)) ? (() => {
                              const exponentNegative_2 = (exponentSign === 45);
                              const at_4 = (at_3 + 1);
                              return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join27_next", ["at"]: at_4, ["exponentNegative"]: exponentNegative_2 });
                            })() : ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join27_next", ["at"]: at_3, ["exponentNegative"]: exponentNegative }));
                            switch (source_ml_source2_s22.$) {
                              case "ml_source1_temporary_convertJavaScriptStringNumberUnits_join27_next": {
                                const at_5 = source_ml_source2_s22["at"];
                                const exponentNegative_3 = source_ml_source2_s22["exponentNegative"];
                                const first = at_5;
                                const source_ml_source2_s23 = source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28(units, end_2, at_5, exponent, exponentLimit);
                                switch (source_ml_source2_s23.$) {
                                  case "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_done": {
                                    const at_6 = source_ml_source2_s23["at"];
                                    const exponent_2 = source_ml_source2_s23["exponent"];
                                    if ((at_6 === first)) {
                                      return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_return", ["ml_source1_temporary_result26"]: NaN });
                                    } else {
                                      if (exponentNegative_3) {
                                        const exponent_3 = -exponent_2;
                                        return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next", ["at"]: at_6, ["exponent"]: exponent_3 });
                                      } else {
                                        return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next", ["at"]: at_6, ["exponent"]: exponent_2 });
                                      }
                                    }
                                  }
                                  default: {
                                    throw new TypeError(`unexpected constructor ${source_ml_source2_s23.$}`);
                                  }
                                }
                              }
                              default: {
                                throw new TypeError(`unexpected constructor ${source_ml_source2_s22.$}`);
                              }
                            }
                          })() : ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next", ["at"]: at_2, ["exponent"]: exponent }));
                          switch (source_ml_source2_s21.$) {
                            case "ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next": {
                              const at_7 = source_ml_source2_s21["at"];
                              const exponent_4 = source_ml_source2_s21["exponent"];
                              if ((at_7 !== end_2)) {
                                return NaN;
                              } else {
                                if ((significant_2 === 0)) {
                                  if (negative_3) {
                                    return (-0);
                                  } else {
                                    return 0;
                                  }
                                } else {
                                  const exponent_5 = (exponent_4 - fraction_2);
                                  const order = (significant_2 + exponent_5);
                                  if ((order > 310)) {
                                    if (negative_3) {
                                      return (-Infinity);
                                    } else {
                                      return Infinity;
                                    }
                                  } else {
                                    if ((order < (-325))) {
                                      if (negative_3) {
                                        return (-0);
                                      } else {
                                        return 0;
                                      }
                                    } else {
                                      const mantissaEnd = decimalEnd;
                                      const source_ml_source2_s24 = source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25(units, start_6, exponent_5, mantissaEnd);
                                      switch (source_ml_source2_s24.$) {
                                        case "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done": {
                                          const exponent_6 = source_ml_source2_s24["exponent"];
                                          const mantissaEnd_2 = source_ml_source2_s24["mantissaEnd"];
                                          const mantissa = 0n;
                                          const index_3 = start_6;
                                          const mantissa_2 = source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop26(units, integerDigits, mantissaEnd_2, mantissa, index_3);
                                          const numerator = ((exponent_6 >= 0) ? (mantissa_2 * readStringNumberDecimalPower(exponent_6)) : mantissa_2);
                                          const denominator = ((exponent_6 < 0) ? readStringNumberDecimalPower(-exponent_6) : 1n);
                                          const value_3 = convertStringNumberRational(numerator, denominator);
                                          if (negative_3) {
                                            return -value_3;
                                          } else {
                                            return value_3;
                                          }
                                        }
                                        default: {
                                          throw new TypeError(`unexpected constructor ${source_ml_source2_s24.$}`);
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                            case "ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_return": {
                              const source_ml_source1_temporary_result26 = source_ml_source2_s21["ml_source1_temporary_result26"];
                              return source_ml_source1_temporary_result26;
                            }
                            default: {
                              throw new TypeError(`unexpected constructor ${source_ml_source2_s21.$}`);
                            }
                          }
                        }
                      }
                      default: {
                        throw new TypeError(`unexpected constructor ${source_ml_source2_s18.$}`);
                      }
                    }
                  }
                  case "ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return": {
                    const source_ml_source1_temporary_result28 = source_ml_source2_s16["ml_source1_temporary_result28"];
                    return source_ml_source1_temporary_result28;
                  }
                  default: {
                    throw new TypeError(`unexpected constructor ${source_ml_source2_s16.$}`);
                  }
                }
              }
            }
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source2_s14.$}`);
          }
        }
      }
    }
    case "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_return": {
      const source_ml_source1_temporary_result29_2 = source_ml_source2_s13["ml_source1_temporary_result29"];
      return source_ml_source1_temporary_result29_2;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source2_s13.$}`);
    }
  }
}

function convertJavaScriptBigIntegerNumber(value) {
  if ((value < 0n)) {
    return -convertStringNumberRational(-value, 1n);
  } else {
    return convertStringNumberRational(value, 1n);
  }
}

function convertJavaScriptUtf8StringNumberUnits(bytes) {
  return convertJavaScriptStringNumberUnits(readJavaScriptUtf8StringUnits(bytes));
}

function readJavaScriptNumberBuiltinOperation(name) {
  if ((name === "Number")) {
    return "convertJavaScriptNumber";
  } else {
    if ((name === "isFinite")) {
      return "testCoercedJavaScriptNumberFinite";
    } else {
      if ((name === "isNaN")) {
        return "testCoercedJavaScriptNumberNaN";
      } else {
        return "";
      }
    }
  }
}

function readJavaScriptNumberConversionResult(operation) {
  if (((operation === "convertJavaScriptNumber") || (operation === "coerceJavaScriptNumber"))) {
    return "float";
  } else {
    if (((operation === "testCoercedJavaScriptNumberFinite") || (operation === "testCoercedJavaScriptNumberNaN"))) {
      return "bool";
    } else {
      return "";
    }
  }
}

function readJavaScriptNumberConversionForm(kind) {
  if ((((((((kind === "float") || (kind === "bool")) || (kind === "string")) || (kind === "unit")) || (kind === "int")) || (kind === "nat")) || (kind === "option"))) {
    return kind;
  } else {
    return "unsupported";
  }
}

function acceptLinkedJavaScriptNumberConversion(kind, operation) {
  return ((kind === "math") && ((operation === "convertJavaScriptNumber") || (operation === "coerceJavaScriptNumber")));
}

function readJavaScriptNumberConversionFailure(operation, kind) {
  if (((operation === "coerceJavaScriptNumber") && ((kind === "int") || (kind === "nat")))) {
    return "Cannot convert a BigInt value to a number";
  } else {
    return "";
  }
}

function requireJavaScriptNumberConversionUnits(kind, operation, argument) {
  return (acceptLinkedJavaScriptNumberConversion(kind, operation) && (argument === "string"));
}

function readJavaScriptPrimitiveNumberOperand(language, kind, element) {
  if ((language !== "JavaScript")) {
    return "";
  } else {
    if ((((kind === "bool") || (kind === "string")) || (kind === "unit"))) {
      return "coerceJavaScriptNumber";
    } else {
      if (((kind === "option") && (element === "string"))) {
        return "coerceJavaScriptNumber";
      } else {
        return "";
      }
    }
  }
}

function readJavaScriptPrimitiveNumericInferenceKind(kind) {
  if ((((kind === "bool") || (kind === "string")) || (kind === "unit"))) {
    return "float";
  } else {
    return "";
  }
}

function readJavaScriptPrimitiveUnaryOperation(symbol) {
  if ((symbol === "+")) {
    return "coerceJavaScriptNumber";
  } else {
    return "";
  }
}

function readJavaScriptMathematicalConversionForm(language, operation) {
  if ((language !== "JavaScript")) {
    return "";
  } else {
    if ((operation === "powerJavaScriptNumberArguments")) {
      return "binary";
    } else {
      if (((operation === "max") || (operation === "min"))) {
        return "variadic";
      } else {
        if ((((((((operation === "abs") || (operation === "floor")) || (operation === "ceil")) || (operation === "trunc")) || (operation === "round")) || (operation === "sign")) || (operation === "sqrt"))) {
          return "unary";
        } else {
          return "";
        }
      }
    }
  }
}

function renderJavaScriptNumberScalarConversion(format, operation, form, value, utf16) {
  if (((!acceptLinkedJavaScriptNumberConversion("math", operation) || (readJavaScriptNumberConversionForm(form) === "unsupported")) || (form === "option"))) {
    return "";
  } else {
    if ((format === "JavaScript")) {
      if ((operation === "convertJavaScriptNumber")) {
        return (("Number(" + value) + ")");
      } else {
        return (("(+(" + value) + "))");
      }
    } else {
      if (((format === "Lean") || (format === "Rocq"))) {
        if (!!(readJavaScriptNumberConversionFailure(operation, form))) {
          return "";
        } else {
          if ((form === "float")) {
            return value;
          } else {
            if ((form === "unit")) {
              if ((format === "Lean")) {
                return (("(let _ := " + value) + "; ((0 : Float) / 0))");
              } else {
                return (("(let _ := " + value) + " in PrimFloat.nan)");
              }
            } else {
              if ((form === "bool")) {
                if ((format === "Lean")) {
                  return (("(if " + value) + " then (1 : Float) else (0 : Float))");
                } else {
                  return (("(if " + value) + " then 1%float else 0%float)");
                }
              } else {
                if (((form === "int") || (form === "nat"))) {
                  return (("(MLStringNumber.convertInteger " + ((form === "nat") ? ((format === "Lean") ? (("(Int.ofNat " + value) + ")") : (("(Z.of_N " + value) + ")")) : value)) + ")");
                } else {
                  if ((format === "Lean")) {
                    return (("(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] " + value) + "))");
                  } else {
                    return (("(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) " + value) + "))");
                  }
                }
              }
            }
          }
        }
      } else {
        if ((format !== "Rust")) {
          return "";
        } else {
          if ((form === "float")) {
            return value;
          } else {
            if ((form === "bool")) {
              return (("(if " + value) + " { 1.0f64 } else { 0.0f64 })");
            } else {
              if ((form === "unit")) {
                return (("{ let _ = " + value) + "; f64::NAN }");
              } else {
                if (((form === "int") || (form === "nat"))) {
                  if ((operation === "coerceJavaScriptNumber")) {
                    return (("{ let _ = " + value) + "; crate::ml_source_exception::SourceException::from_text(\"TypeError\", \"Cannot convert a BigInt value to a number\").raise() }");
                  } else {
                    return (("crate::ml_string_number::convert_java_script_big_integer_number(" + value) + ")");
                  }
                } else {
                  const units = (utf16 ? (("(" + value) + ").0.iter().copied()") : (("(" + value) + ").encode_utf16()"));
                  return (("crate::ml_string_number::convert_java_script_string_number_units(&" + units) + ".map(f64::from).collect::<Vec<f64>>())");
                }
              }
            }
          }
        }
      }
    }
  }
}

function readJavaScriptNumberConversionStatement() {
  return "ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws";
}

function convertJavaScriptStringBigIntegerUnits(units) {
  const index = 0;
  const source_ml_source2_s25 = source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30(units, index);
  switch (source_ml_source2_s25.$) {
    case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_done": {
      const start = 0;
      const end = units.length;
      const start_2 = source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop31(units, start, end);
      const end_2 = source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop32(units, start_2, end);
      if ((start_2 === end_2)) {
        return [0n];
      } else {
        const negative = false;
        const signed = false;
        const sign = ((units)[start_2] ?? (-1));
        const source_ml_source2_s26 = (((sign === 43) || (sign === 45)) ? (() => {
          const negative_2 = (sign === 45);
          const signed_2 = true;
          const start_3 = (start_2 + 1);
          return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33_next", ["start"]: start_3, ["negative"]: negative_2, ["signed"]: signed_2 });
        })() : ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33_next", ["start"]: start_2, ["negative"]: negative, ["signed"]: signed }));
        switch (source_ml_source2_s26.$) {
          case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33_next": {
            const start_4 = source_ml_source2_s26["start"];
            const negative_3 = source_ml_source2_s26["negative"];
            const signed_3 = source_ml_source2_s26["signed"];
            if ((start_4 === end_2)) {
              return [];
            } else {
              const radix = 10;
              const base = 10n;
              const source_ml_source2_s27 = (((!signed_3 && (((units)[start_4] ?? (-1)) === 48)) && ((end_2 - start_4) >= 2)) ? (() => {
                const marker = ((units)[(start_4 + 1)] ?? (-1));
                const selected = (((marker === 120) || (marker === 88)) ? 16 : (((marker === 98) || (marker === 66)) ? 2 : (((marker === 111) || (marker === 79)) ? 8 : 0)));
                if ((selected !== 0)) {
                  const radix_2 = selected;
                  const base_2 = ((selected === 16) ? 16n : ((selected === 8) ? 8n : 2n));
                  const start_5 = (start_4 + 2);
                  return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next", ["start"]: start_5, ["radix"]: radix_2, ["base"]: base_2 });
                } else {
                  return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next", ["start"]: start_4, ["radix"]: radix, ["base"]: base });
                }
              })() : ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next", ["start"]: start_4, ["radix"]: radix, ["base"]: base }));
              switch (source_ml_source2_s27.$) {
                case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next": {
                  const start_6 = source_ml_source2_s27["start"];
                  const radix_3 = source_ml_source2_s27["radix"];
                  const base_3 = source_ml_source2_s27["base"];
                  if ((start_6 === end_2)) {
                    return [];
                  } else {
                    const digits = [0n, 1n, 2n, 3n, 4n, 5n, 6n, 7n, 8n, 9n, 10n, 11n, 12n, 13n, 14n, 15n];
                    const value = 0n;
                    const index_2 = start_6;
                    const source_ml_source2_s28 = source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35(units, end_2, radix_3, base_3, digits, value, index_2);
                    switch (source_ml_source2_s28.$) {
                      case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_done": {
                        const value_2 = source_ml_source2_s28["value"];
                        return [(negative_3 ? -value_2 : value_2)];
                      }
                      case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_return": {
                        const source_ml_source1_temporary_result35 = source_ml_source2_s28["ml_source1_temporary_result35"];
                        return source_ml_source1_temporary_result35;
                      }
                      default: {
                        throw new TypeError(`unexpected constructor ${source_ml_source2_s28.$}`);
                      }
                    }
                  }
                }
                default: {
                  throw new TypeError(`unexpected constructor ${source_ml_source2_s27.$}`);
                }
              }
            }
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source2_s26.$}`);
          }
        }
      }
    }
    case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_return": {
      const source_ml_source1_temporary_result35_2 = source_ml_source2_s25["ml_source1_temporary_result35"];
      return source_ml_source1_temporary_result35_2;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source2_s25.$}`);
    }
  }
}

function convertJavaScriptUtf8BigInteger(bytes) {
  return convertJavaScriptStringBigIntegerUnits(readJavaScriptUtf8StringUnits(bytes));
}

function convertJavaScriptNumberBigInteger(value) {
  if (!Number.isInteger(value)) {
    return [];
  } else {
    const negative = (value < 0);
    const value_2 = Math.abs(value);
    const integer = 0n;
    const weight = 1n;
    const integer_2 = source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop36(value_2, integer, weight);
    return [(negative ? -integer_2 : integer_2)];
  }
}

function convertJavaScriptBooleanBigInteger(value) {
  if (value) {
    return 1n;
  } else {
    return 0n;
  }
}

function compareJavaScriptNumberBigInteger(value, integer) {
  if (Number.isNaN(value)) {
    return 2;
  } else {
    if ((value === Infinity)) {
      return 1;
    } else {
      if ((value === (-Infinity))) {
        return (-1);
      } else {
        const floor = Math.floor(value);
        const lower = ((convertJavaScriptNumberBigInteger(floor))[0] ?? 0n);
        if ((lower < integer)) {
          return (-1);
        } else {
          if ((lower > integer)) {
            return 1;
          } else {
            if ((value === floor)) {
              return 0;
            } else {
              return 1;
            }
          }
        }
      }
    }
  }
}

function compareJavaScriptStringBigIntegerUnits(units, integer) {
  const result = convertJavaScriptStringBigIntegerUnits(units);
  if ((result.length !== 1)) {
    return 2;
  } else {
    const value = ((result)[0] ?? 0n);
    if ((value < integer)) {
      return (-1);
    } else {
      if ((value > integer)) {
        return 1;
      } else {
        return 0;
      }
    }
  }
}

function compareJavaScriptUtf8BigInteger(bytes, integer) {
  return compareJavaScriptStringBigIntegerUnits(readJavaScriptUtf8StringUnits(bytes), integer);
}

function divideJavaScriptBigInteger(left, right) {
  if ((right === 0n)) {
    return [];
  } else {
    const remaining = ((left < 0n) ? -left : left);
    const divisor = ((right < 0n) ? -right : right);
    const shifted = divisor;
    const weight = 1n;
    const quotient = 0n;
    const source_ml_source2_s29 = source_ml_source1_temporary_divideJavaScriptBigInteger_loop37(remaining, shifted, weight);
    switch (source_ml_source2_s29.$) {
      case "ml_source1_temporary_divideJavaScriptBigInteger_loop37_done": {
        const shifted_2 = source_ml_source2_s29["shifted"];
        const weight_2 = source_ml_source2_s29["weight"];
        const source_ml_source2_s30 = source_ml_source1_temporary_divideJavaScriptBigInteger_loop38(remaining, shifted_2, weight_2, quotient);
        switch (source_ml_source2_s30.$) {
          case "ml_source1_temporary_divideJavaScriptBigInteger_loop38_done": {
            const remaining_2 = source_ml_source2_s30["remaining"];
            const quotient_2 = source_ml_source2_s30["quotient"];
            return [(((left < 0n) !== (right < 0n)) ? -quotient_2 : quotient_2), ((left < 0n) ? -remaining_2 : remaining_2)];
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source2_s30.$}`);
          }
        }
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source2_s29.$}`);
      }
    }
  }
}

function powerJavaScriptBigInteger(base, exponent) {
  if ((exponent < 0n)) {
    return [];
  } else {
    const result = 1n;
    const weight = base;
    const remaining = exponent;
    const result_2 = source_ml_source1_temporary_powerJavaScriptBigInteger_loop39(result, weight, remaining);
    return [result_2];
  }
}

function normalizeNumberPowerBound(value, exponent, scale, upward) {
  if ((value <= 0n)) {
    return [0n, 0n];
  } else {
    const source_ml_source2_s31 = source_ml_source1_temporary_normalizeNumberPowerBound_loop40(value, exponent, scale, upward);
    switch (source_ml_source2_s31.$) {
      case "ml_source1_temporary_normalizeNumberPowerBound_loop40_done": {
        const value_2 = source_ml_source2_s31["value"];
        const exponent_2 = source_ml_source2_s31["exponent"];
        const source_ml_source2_s32 = source_ml_source1_temporary_normalizeNumberPowerBound_loop41(value_2, exponent_2, scale);
        switch (source_ml_source2_s32.$) {
          case "ml_source1_temporary_normalizeNumberPowerBound_loop41_done": {
            const value_3 = source_ml_source2_s32["value"];
            const exponent_3 = source_ml_source2_s32["exponent"];
            return [value_3, exponent_3];
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source2_s32.$}`);
          }
        }
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source2_s31.$}`);
      }
    }
  }
}

function divideNumberPowerBound(numerator, denominator, upward) {
  const result = divideJavaScriptBigInteger(numerator, denominator);
  const quotient = ((result)[0] ?? 0n);
  const remainder = ((result)[1] ?? 0n);
  return (quotient + ((upward && (remainder !== 0n)) ? 1n : 0n));
}

function rootNumberPowerInteger(value) {
  if ((value <= 0n)) {
    return 0n;
  } else {
    const root = 1n;
    const remaining = value;
    const root_2 = source_ml_source1_temporary_rootNumberPowerInteger_loop42(root, remaining);
    const root_3 = (root_2 * 2n);
    return source_ml_source1_temporary_rootNumberPowerInteger_loop43(value, root_3);
  }
}

function multiplyNumberPowerInterval(left, right, scale) {
  const lower = normalizeNumberPowerBound(divideNumberPowerBound((((left)[0] ?? 0n) * ((right)[0] ?? 0n)), scale, false), (((left)[1] ?? 0n) + ((right)[1] ?? 0n)), scale, false);
  const upper = normalizeNumberPowerBound(divideNumberPowerBound((((left)[2] ?? 0n) * ((right)[2] ?? 0n)), scale, true), (((left)[3] ?? 0n) + ((right)[3] ?? 0n)), scale, true);
  return [((lower)[0] ?? 0n), ((lower)[1] ?? 0n), ((upper)[0] ?? 0n), ((upper)[1] ?? 0n)];
}

function rootNumberPowerBound(value, exponent, scale, upward) {
  const source_ml_source2_s33 = ((ml_divide(exponent, 2n, 'trunc', "Division by zero", true) !== 0n) ? (() => {
    const value_2 = (value * 2n);
    const exponent_2 = (exponent - 1n);
    return ({ $: "ml_source1_temporary_rootNumberPowerBound_join44_next", ["value"]: value_2, ["exponent"]: exponent_2 });
  })() : ({ $: "ml_source1_temporary_rootNumberPowerBound_join44_next", ["value"]: value, ["exponent"]: exponent }));
  switch (source_ml_source2_s33.$) {
    case "ml_source1_temporary_rootNumberPowerBound_join44_next": {
      const value_3 = source_ml_source2_s33["value"];
      const exponent_3 = source_ml_source2_s33["exponent"];
      const square = (value_3 * scale);
      const root = rootNumberPowerInteger(square);
      const root_3 = ((upward && ((root * root) < square)) ? (() => {
        const root_2 = (root + 1n);
        return root_2;
      })() : root);
      return normalizeNumberPowerBound(root_3, ml_divide(exponent_3, 2n, 'trunc', "Division by zero", false), scale, upward);
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source2_s33.$}`);
    }
  }
}

function rootNumberPowerInterval(interval, scale) {
  const lower = rootNumberPowerBound(((interval)[0] ?? 0n), ((interval)[1] ?? 0n), scale, false);
  const upper = rootNumberPowerBound(((interval)[2] ?? 0n), ((interval)[3] ?? 0n), scale, true);
  return [((lower)[0] ?? 0n), ((lower)[1] ?? 0n), ((upper)[0] ?? 0n), ((upper)[1] ?? 0n)];
}

function invertNumberPowerInterval(interval, scale) {
  const lower = normalizeNumberPowerBound(divideNumberPowerBound((scale * scale), ((interval)[2] ?? 0n), false), -((interval)[3] ?? 0n), scale, false);
  const upper = normalizeNumberPowerBound(divideNumberPowerBound((scale * scale), ((interval)[0] ?? 0n), true), -((interval)[1] ?? 0n), scale, true);
  return [((lower)[0] ?? 0n), ((lower)[1] ?? 0n), ((upper)[0] ?? 0n), ((upper)[1] ?? 0n)];
}

function decomposeNumberPowerMagnitude(value) {
  const exponent = 0n;
  const source_ml_source2_s34 = source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45(value, exponent);
  switch (source_ml_source2_s34.$) {
    case "ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_done": {
      const value_2 = source_ml_source2_s34["value"];
      const exponent_2 = source_ml_source2_s34["exponent"];
      const source_ml_source2_s35 = source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46(value_2, exponent_2);
      switch (source_ml_source2_s35.$) {
        case "ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_done": {
          const value_3 = source_ml_source2_s35["value"];
          const exponent_3 = source_ml_source2_s35["exponent"];
          return [((convertJavaScriptNumberBigInteger((value_3 * 4503599627370496)))[0] ?? 0n), (exponent_3 - 52n)];
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source2_s35.$}`);
        }
      }
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source2_s34.$}`);
    }
  }
}

function roundNumberPowerBound(value, exponent, precision) {
  if (((value === 0n) || (exponent < (-1076n)))) {
    return 0;
  } else {
    if ((exponent > 1024n)) {
      return Infinity;
    } else {
      const shift = (exponent - precision);
      if ((shift >= 0n)) {
        return convertStringNumberRational((value * ((powerJavaScriptBigInteger(2n, shift))[0] ?? 0n)), 1n);
      } else {
        return convertStringNumberRational(value, ((powerJavaScriptBigInteger(2n, -shift))[0] ?? 0n));
      }
    }
  }
}

function powerJavaScriptNumber(base, exponent) {
  if (Number.isNaN(exponent)) {
    return NaN;
  } else {
    if ((exponent === 0)) {
      return 1;
    } else {
      if (Number.isNaN(base)) {
        return NaN;
      } else {
        const magnitude = Math.abs(base);
        if (!Number.isFinite(exponent)) {
          if ((magnitude === 1)) {
            return NaN;
          } else {
            if ((magnitude > 1)) {
              if ((exponent > 0)) {
                return Infinity;
              } else {
                return 0;
              }
            } else {
              if ((exponent > 0)) {
                return 0;
              } else {
                return Infinity;
              }
            }
          }
        } else {
          const odd = (Number.isInteger(exponent) && ((Math.abs(exponent) % 2) === 1));
          const negative = (((base < 0) || ((base === 0) && ((1 / base) < 0))) && odd);
          if ((base === 0)) {
            const value = ((exponent > 0) ? 0 : Infinity);
            if (negative) {
              return -value;
            } else {
              return value;
            }
          } else {
            if (!Number.isFinite(base)) {
              const value_2 = ((exponent > 0) ? Infinity : 0);
              if (negative) {
                return -value_2;
              } else {
                return value_2;
              }
            } else {
              if (((base < 0) && !Number.isInteger(exponent))) {
                return NaN;
              } else {
                if ((magnitude === 1)) {
                  if (negative) {
                    return (-1);
                  } else {
                    return 1;
                  }
                } else {
                  const input = decomposeNumberPowerMagnitude(magnitude);
                  const power = decomposeNumberPowerMagnitude(Math.abs(exponent));
                  const precision = 192n;
                  return source_ml_source1_temporary_powerJavaScriptNumber_loop47(exponent, negative, input, power, precision);
                }
              }
            }
          }
        }
      }
    }
  }
}

function acceptJavaScriptExponentiationBase(prefix, parenthesized) {
  return (parenthesized || !(["+", "-", "!", "~", "typeof", "void", "delete", "await"]).includes(prefix, 0));
}

function readJavaScriptBigIntegerExponentMessage() {
  return "Exponent must be positive";
}

function renderJavaScriptBigIntegerPower(target, base, exponent) {
  if ((target === "JavaScript")) {
    return (((("ml_big_integer_conversion.power(" + base) + ",") + exponent) + ")");
  } else {
    if ((target === "Rust")) {
      return (((("crate::ml_big_integer_conversion::power_java_script_big_integer(" + base) + ",") + exponent) + ")");
    } else {
      if (((target === "Lean") || (target === "Rocq"))) {
        return (((("(MLBigIntegerConversion.power " + base) + " ") + exponent) + ")");
      } else {
        return "";
      }
    }
  }
}

function acceptNonzeroJavaScriptIntegerLiteral(value) {
  return (((value !== "") && (value !== "0")) && (value !== "-0"));
}

function requireJavaScriptBigIntegerDivision(language, operation, kind, literal) {
  return ((((language === "JavaScript") && (kind === "int")) && ((operation === "div") || (operation === "rem"))) && !acceptNonzeroJavaScriptIntegerLiteral(literal));
}

function readJavaScriptBigIntegerDivisionMessage() {
  return "Division by zero";
}

function renderJavaScriptBigIntegerDivision(target, left, right) {
  if ((target === "JavaScript")) {
    return (((("ml_big_integer_conversion.divide(" + left) + ",") + right) + ")");
  } else {
    if ((target === "Rust")) {
      return (((("crate::ml_big_integer_conversion::divide_java_script_big_integer(" + left) + ",") + right) + ")");
    } else {
      if (((target === "Lean") || (target === "Rocq"))) {
        return (((("(MLBigIntegerConversion.divide " + left) + " ") + right) + ")");
      } else {
        return "";
      }
    }
  }
}

function readJavaScriptBigIntegerConversionForm(kind) {
  if (((kind === "int") || (kind === "nat"))) {
    return "identity";
  } else {
    if ((kind === "float")) {
      return "number";
    } else {
      if ((kind === "string")) {
        return "string";
      } else {
        if ((kind === "bool")) {
          return "boolean";
        } else {
          if ((kind === "unit")) {
            return "undefined";
          } else {
            if ((kind === "option")) {
              return "optional";
            } else {
              return "unsupported";
            }
          }
        }
      }
    }
  }
}

function readJavaScriptBigIntegerConversionErrorClass(form) {
  if ((form === "string")) {
    return "SyntaxError";
  } else {
    if ((form === "number")) {
      return "RangeError";
    } else {
      if (((form === "undefined") || (form === "null"))) {
        return "TypeError";
      } else {
        return "";
      }
    }
  }
}

function renderJavaScriptBigIntegerConversionMessage(form, value) {
  const pieces = readJavaScriptBigIntegerConversionMessagePieces(form);
  if ((pieces.length === 2)) {
    return ((((pieces)[0] ?? "") + value) + ((pieces)[1] ?? ""));
  } else {
    return "";
  }
}

function readJavaScriptBigIntegerConversionMessagePieces(form) {
  if ((form === "number")) {
    return ["The number ", " cannot be converted to a BigInt because it is not an integer"];
  } else {
    if ((((form === "string") || (form === "undefined")) || (form === "null"))) {
      return ["Cannot convert ", " to a BigInt"];
    } else {
      return [];
    }
  }
}

function readJavaScriptBigIntegerBuiltinOperation(name) {
  if ((name === "BigInt")) {
    return "convertJavaScriptBigInteger";
  } else {
    return "";
  }
}

function readJavaScriptBigIntegerResultOperation(operation) {
  if ((operation === "compareJavaScriptNumberBigIntegerValue")) {
    return "number-compare";
  } else {
    if ((operation === "compareJavaScriptStringBigIntegerValue")) {
      return "string-compare";
    } else {
      if ((operation === "powerJavaScriptNumberValue")) {
        return "number-power";
      } else {
        if ((operation === "powerJavaScriptBigIntegerResult")) {
          return "power";
        } else {
          if ((operation === "divideJavaScriptBigIntegerResult")) {
            return "division";
          } else {
            if ((operation === "parseJavaScriptStringBigInteger")) {
              return "string";
            } else {
              if ((operation === "convertJavaScriptBooleanBigIntegerValue")) {
                return "boolean";
              } else {
                if ((operation === "convertJavaScriptNumberBigIntegerResult")) {
                  return "number";
                } else {
                  return "";
                }
              }
            }
          }
        }
      }
    }
  }
}

function acceptLinkedJavaScriptBigIntegerResult(kind, operation) {
  return ((kind === "math") && (readJavaScriptBigIntegerResultOperation(operation) !== ""));
}

function readJavaScriptBigIntegerIntrinsicOperandKinds(operation) {
  const form = readJavaScriptBigIntegerResultOperation(operation);
  if (((form === "division") || (form === "power"))) {
    return ["int", "int"];
  } else {
    if ((form === "number-power")) {
      return ["float", "float"];
    } else {
      if ((form === "number-compare")) {
        return ["float", "int"];
      } else {
        if ((form === "string-compare")) {
          return ["string", "int"];
        } else {
          if ((form === "string")) {
            return ["string"];
          } else {
            if ((form === "number")) {
              return ["float"];
            } else {
              if ((form === "boolean")) {
                return ["bool"];
              } else {
                return [];
              }
            }
          }
        }
      }
    }
  }
}

function readJavaScriptBigIntegerIntrinsicResultKind(operation) {
  const form = readJavaScriptBigIntegerResultOperation(operation);
  if ((((form === "number-power") || (form === "number-compare")) || (form === "string-compare"))) {
    return "float";
  } else {
    if ((form === "boolean")) {
      return "int";
    } else {
      if ((form === "")) {
        return "";
      } else {
        return "array-integer";
      }
    }
  }
}

function renderJavaScriptBigIntegerComparison(target, form, left, right, utf16) {
  if ((form === "number-compare")) {
    if ((target === "JavaScript")) {
      return (((("ml_big_integer_conversion.compareNumber(" + left) + ",") + right) + ")");
    } else {
      if ((target === "Rust")) {
        return (((("crate::ml_big_integer_conversion::compare_java_script_number_big_integer(" + left) + ",") + right) + ")");
      } else {
        if (((target === "Lean") || (target === "Rocq"))) {
          return (((("(MLBigIntegerConversion.compareNumber " + left) + " ") + right) + ")");
        } else {
          return "";
        }
      }
    }
  } else {
    if ((form !== "string-compare")) {
      return "";
    } else {
      if ((target === "JavaScript")) {
        return (((("((ml_text,ml_integer)=>ml_big_integer_conversion.compareUnits(Array.from({length:ml_text.length},(_,index)=>ml_text.charCodeAt(index)),ml_integer))(" + left) + ",") + right) + ")");
      } else {
        if ((target === "Rust")) {
          return (((("crate::ml_big_integer_conversion::compare_java_script_string_big_integer_units(&" + (utf16 ? (("(" + left) + ").0.iter().copied()") : (("(" + left) + ").encode_utf16()"))) + ".map(f64::from).collect::<Vec<f64>>(),") + right) + ")");
        } else {
          if ((target === "Lean")) {
            return (((("(MLBigIntegerConversion.compareUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] " + left) + ") ") + right) + ")");
          } else {
            if ((target === "Rocq")) {
              return (((("(MLBigIntegerConversion.compareUtf8 ((fix ml_compare_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_compare_bytes rest) end) " + left) + ") ") + right) + ")");
            } else {
              return "";
            }
          }
        }
      }
    }
  }
}

function renderJavaScriptNumberPower(target, base, exponent) {
  if ((target === "JavaScript")) {
    return (((("ml_big_integer_conversion.powerNumber(" + base) + ",") + exponent) + ")");
  } else {
    if ((target === "Rust")) {
      return (((("crate::ml_big_integer_conversion::power_java_script_number(" + base) + ",") + exponent) + ")");
    } else {
      if (((target === "Lean") || (target === "Rocq"))) {
        return (((("(MLBigIntegerConversion.powerNumber " + base) + " ") + exponent) + ")");
      } else {
        return "";
      }
    }
  }
}

function renderJavaScriptBigIntegerResultExpression(target, form, value, utf16) {
  if ((form === "boolean")) {
    if ((target === "JavaScript")) {
      return (("ml_big_integer_conversion.convertBoolean(" + value) + ")");
    } else {
      if ((target === "Rust")) {
        return (("crate::ml_big_integer_conversion::convert_java_script_boolean_big_integer(" + value) + ")");
      } else {
        if (((target === "Lean") || (target === "Rocq"))) {
          return (("(MLBigIntegerConversion.convertBoolean " + value) + ")");
        } else {
          return "";
        }
      }
    }
  } else {
    if (((form !== "string") && (form !== "number"))) {
      return "";
    } else {
      if ((target === "JavaScript")) {
        if ((form === "number")) {
          return (("ml_big_integer_conversion.convertNumber(" + value) + ")");
        } else {
          return (("((ml_text)=>ml_big_integer_conversion.convertUnits(Array.from({length:ml_text.length}, (_,index)=>ml_text.charCodeAt(index))))(" + value) + ")");
        }
      } else {
        if ((target === "Rust")) {
          if ((form === "number")) {
            return (("crate::ml_big_integer_conversion::convert_java_script_number_big_integer(" + value) + ")");
          } else {
            const units = (utf16 ? (("(" + value) + ").0.iter().copied()") : (("(" + value) + ").encode_utf16()"));
            return (("crate::ml_big_integer_conversion::convert_java_script_string_big_integer_units(&" + units) + ".map(f64::from).collect::<Vec<f64>>())");
          }
        } else {
          if ((target === "Lean")) {
            if ((form === "number")) {
              return (("(MLBigIntegerConversion.convertNumber " + value) + ")");
            } else {
              return (("(MLBigIntegerConversion.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] " + value) + "))");
            }
          } else {
            if ((target === "Rocq")) {
              if ((form === "number")) {
                return (("(MLBigIntegerConversion.convertNumber " + value) + ")");
              } else {
                return (("(MLBigIntegerConversion.convertUtf8 ((fix ml_integer_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_integer_bytes rest) end) " + value) + "))");
              }
            } else {
              return "";
            }
          }
        }
      }
    }
  }
}

function source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4(bytes, index, scalar, remaining) {
  for (;;) {
    if ((remaining > 0)) {
      const next = ((bytes)[index] ?? (-1));
      if (((next < 128) || (next > 191))) {
        return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_return", ["ml_source1_temporary_result4"]: [(-1)] });
      } else {
        const scalar_2 = (((scalar * 64) + next) - 128);
        const index_2 = (index + 1);
        const remaining_2 = (remaining - 1);
        [bytes, index, scalar, remaining] = [bytes, index_2, scalar_2, remaining_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_done", ["index"]: index, ["scalar"]: scalar });
    }
  }
}

function source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2(bytes, units, index) {
  for (;;) {
    if ((index < bytes.length)) {
      const scalar = ((bytes)[index] ?? (-1));
      const index_2 = (index + 1);
      const remaining = 0;
      const minimum = 0;
      const source_ml_source2_s36 = (((scalar >= 194) && (scalar <= 223)) ? (() => {
        const scalar_2 = (scalar - 192);
        const remaining_2 = 1;
        const minimum_2 = 128;
        return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next", ["scalar"]: scalar_2, ["remaining"]: remaining_2, ["minimum"]: minimum_2 });
      })() : (((scalar >= 224) && (scalar <= 239)) ? (() => {
        const scalar_3 = (scalar - 224);
        const remaining_3 = 2;
        const minimum_3 = 2048;
        return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next", ["scalar"]: scalar_3, ["remaining"]: remaining_3, ["minimum"]: minimum_3 });
      })() : (((scalar >= 240) && (scalar <= 244)) ? (() => {
        const scalar_4 = (scalar - 240);
        const remaining_4 = 3;
        const minimum_4 = 65536;
        return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next", ["scalar"]: scalar_4, ["remaining"]: remaining_4, ["minimum"]: minimum_4 });
      })() : ((scalar >= 128) ? ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_return", ["ml_source1_temporary_result4"]: [(-1)] }) : ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next", ["scalar"]: scalar, ["remaining"]: remaining, ["minimum"]: minimum })))));
      switch (source_ml_source2_s36.$) {
        case "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next": {
          const scalar_5 = source_ml_source2_s36["scalar"];
          const remaining_5 = source_ml_source2_s36["remaining"];
          const minimum_5 = source_ml_source2_s36["minimum"];
          const source_ml_source2_s37 = source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4(bytes, index_2, scalar_5, remaining_5);
          switch (source_ml_source2_s37.$) {
            case "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_done": {
              const index_3 = source_ml_source2_s37["index"];
              const scalar_6 = source_ml_source2_s37["scalar"];
              if ((((scalar_6 < minimum_5) || (scalar_6 > 1114111)) || ((scalar_6 >= 55296) && (scalar_6 <= 57343)))) {
                return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_source1_temporary_result2"]: [(-1)] });
              } else {
                if ((scalar_6 <= 65535)) {
                  const units_2 = [...units, ...[scalar_6]];
                  [bytes, units, index] = [bytes, units_2, index_3];
                  continue;
                } else {
                  const units_3 = [...units, ...[(55296 + Math.floor(((scalar_6 - 65536) / 1024)))]];
                  const units_4 = [...units_3, ...[(56320 + ((scalar_6 - 65536) % 1024))]];
                  [bytes, units, index] = [bytes, units_4, index_3];
                  continue;
                }
              }
            }
            case "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_return": {
              const source_ml_source1_temporary_result4 = source_ml_source2_s37["ml_source1_temporary_result4"];
              return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_source1_temporary_result2"]: source_ml_source1_temporary_result4 });
            }
            default: {
              throw new TypeError(`unexpected constructor ${source_ml_source2_s37.$}`);
            }
          }
        }
        case "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_return": {
          const source_ml_source1_temporary_result4_2 = source_ml_source2_s36["ml_source1_temporary_result4"];
          return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_source1_temporary_result2"]: source_ml_source1_temporary_result4_2 });
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source2_s36.$}`);
        }
      }
    } else {
      return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_done", ["units"]: units });
    }
  }
}

function source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1(bytes, index) {
  for (;;) {
    if ((index < bytes.length)) {
      const unit = ((bytes)[index] ?? (-1));
      if (((!Number.isSafeInteger(unit) || (unit < 0)) || (unit > 255))) {
        return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_return", ["ml_source1_temporary_result4"]: [(-1)] });
      } else {
        const index_2 = (index + 1);
        [bytes, index] = [bytes, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_done" });
    }
  }
}

function source_ml_source1_temporary_readStringNumberDecimalPower_loop5(exponent, value, base) {
  for (;;) {
    if ((exponent > 0)) {
      const value_3 = (((exponent % 2) === 1) ? (() => {
        const value_2 = (value * base);
        return value_2;
      })() : value);
      const exponent_2 = Math.floor((exponent / 2));
      if ((exponent_2 > 0)) {
        const base_2 = (base * base);
        [exponent, value, base] = [exponent_2, value_3, base_2];
        continue;
      } else {
        [exponent, value, base] = [exponent_2, value_3, base];
        continue;
      }
    } else {
      return value;
    }
  }
}

function source_ml_source1_temporary_readStringNumberExactMantissa_loop6(value, result, scale) {
  for (;;) {
    if ((value > 0n)) {
      const result_3 = ((ml_divide(value, 2n, 'trunc', "Division by zero", true) === 1n) ? (() => {
        const result_2 = (result + scale);
        return result_2;
      })() : result);
      const value_2 = ml_divide(value, 2n, 'trunc', "Division by zero", false);
      const scale_2 = (scale * 2);
      [value, result, scale] = [value_2, result_3, scale_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_source1_temporary_readStringNumberBinaryPower_loop8(exponent, result) {
  for (;;) {
    if ((exponent < 0)) {
      const result_2 = (result / 2);
      const exponent_2 = (exponent + 1);
      [exponent, result] = [exponent_2, result_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_source1_temporary_readStringNumberBinaryPower_loop7(exponent, result) {
  for (;;) {
    if ((exponent > 0)) {
      const result_2 = (result * 2);
      const exponent_2 = (exponent - 1);
      [exponent, result] = [exponent_2, result_2];
      continue;
    } else {
      return ({ $: "ml_source1_temporary_readStringNumberBinaryPower_loop7_done", ["exponent"]: exponent, ["result"]: result });
    }
  }
}

function source_ml_source1_temporary_convertStringNumberRational_loop12(quotient, remainder, scale, weight) {
  for (;;) {
    if ((weight > 1n)) {
      const scale_2 = ml_divide(scale, 2n, 'trunc', "Division by zero", false);
      const weight_2 = ml_divide(weight, 2n, 'trunc', "Division by zero", false);
      if ((remainder >= scale_2)) {
        const remainder_2 = (remainder - scale_2);
        const quotient_2 = (quotient + weight_2);
        [quotient, remainder, scale, weight] = [quotient_2, remainder_2, scale_2, weight_2];
        continue;
      } else {
        [quotient, remainder, scale, weight] = [quotient, remainder, scale_2, weight_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertStringNumberRational_loop12_done", ["quotient"]: quotient, ["remainder"]: remainder });
    }
  }
}

function source_ml_source1_temporary_convertStringNumberRational_loop11(remainder, scale, weight) {
  for (;;) {
    if ((scale <= remainder)) {
      const scale_2 = (scale * 2n);
      const weight_2 = (weight * 2n);
      [remainder, scale, weight] = [remainder, scale_2, weight_2];
      continue;
    } else {
      return ({ $: "ml_source1_temporary_convertStringNumberRational_loop11_done", ["scale"]: scale, ["weight"]: weight });
    }
  }
}

function source_ml_source1_temporary_convertStringNumberRational_loop13(numerator, shift, index) {
  for (;;) {
    if ((index < shift)) {
      const numerator_2 = (numerator * 2n);
      const index_2 = (index + 1);
      [numerator, shift, index] = [numerator_2, shift, index_2];
      continue;
    } else {
      return numerator;
    }
  }
}

function source_ml_source1_temporary_convertStringNumberRational_loop14(denominator, shift, index) {
  for (;;) {
    if ((index < -shift)) {
      const denominator_2 = (denominator * 2n);
      const index_2 = (index + 1);
      [denominator, shift, index] = [denominator_2, shift, index_2];
      continue;
    } else {
      return denominator;
    }
  }
}

function source_ml_source1_temporary_convertStringNumberRational_loop15(numerator, exponent, scale) {
  for (;;) {
    if ((numerator >= (scale * 2n))) {
      const scale_2 = (scale * 2n);
      const exponent_2 = (exponent + 1);
      if ((exponent_2 >= 1024)) {
        return ({ $: "ml_source1_temporary_convertStringNumberRational_loop15_return", ["ml_source1_temporary_result15"]: Infinity });
      } else {
        [numerator, exponent, scale] = [numerator, exponent_2, scale_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertStringNumberRational_loop15_done", ["exponent"]: exponent });
    }
  }
}

function source_ml_source1_temporary_convertStringNumberRational_loop16(denominator, exponent, scale) {
  for (;;) {
    if ((scale < denominator)) {
      const scale_2 = (scale * 2n);
      const exponent_2 = (exponent - 1);
      if ((exponent_2 < (-1075))) {
        return ({ $: "ml_source1_temporary_convertStringNumberRational_loop16_return", ["ml_source1_temporary_result16"]: 0 });
      } else {
        [denominator, exponent, scale] = [denominator, exponent_2, scale_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertStringNumberRational_loop16_done", ["exponent"]: exponent });
    }
  }
}

function source_ml_source1_temporary_acceptStringNumberInfinity_loop17(units, start, expected, index) {
  for (;;) {
    if ((index < expected.length)) {
      if ((((units)[(start + index)] ?? (-1)) !== ((expected)[index] ?? (-2)))) {
        return ({ $: "ml_source1_temporary_acceptStringNumberInfinity_loop17_return", ["ml_source1_temporary_result17"]: false });
      } else {
        const index_2 = (index + 1);
        [units, start, expected, index] = [units, start, expected, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_acceptStringNumberInfinity_loop17_done" });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19(units, start, end) {
  for (;;) {
    if (((start < end) && acceptJavaScriptWhitespaceUnit(((units)[start] ?? (-1))))) {
      const start_2 = (start + 1);
      [units, start, end] = [units, start_2, end];
      continue;
    } else {
      return start;
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop20(units, start, end) {
  for (;;) {
    if (((end > start) && acceptJavaScriptWhitespaceUnit(((units)[(end - 1)] ?? (-1))))) {
      const end_2 = (end - 1);
      [units, start, end] = [units, start, end_2];
      continue;
    } else {
      return end;
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop26(units, integerDigits, mantissaEnd, mantissa, index) {
  for (;;) {
    if ((index < mantissaEnd)) {
      const unit = ((units)[index] ?? (-1));
      if ((unit !== 46)) {
        const mantissa_2 = ((mantissa * 10n) + ((integerDigits)[(unit - 48)] ?? 0n));
        const index_2 = (index + 1);
        [units, integerDigits, mantissaEnd, mantissa, index] = [units, integerDigits, mantissaEnd, mantissa_2, index_2];
        continue;
      } else {
        const index_3 = (index + 1);
        [units, integerDigits, mantissaEnd, mantissa, index] = [units, integerDigits, mantissaEnd, mantissa, index_3];
        continue;
      }
    } else {
      return mantissa;
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25(units, start, exponent, mantissaEnd) {
  for (;;) {
    if ((mantissaEnd > start)) {
      const unit = ((units)[(mantissaEnd - 1)] ?? (-1));
      if ((unit === 46)) {
        const mantissaEnd_2 = (mantissaEnd - 1);
        [units, start, exponent, mantissaEnd] = [units, start, exponent, mantissaEnd_2];
        continue;
      } else {
        if ((unit !== 48)) {
          return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done", ["exponent"]: exponent, ["mantissaEnd"]: mantissaEnd });
        } else {
          const mantissaEnd_3 = (mantissaEnd - 1);
          const exponent_2 = (exponent + 1);
          [units, start, exponent, mantissaEnd] = [units, start, exponent_2, mantissaEnd_3];
          continue;
        }
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done", ["exponent"]: exponent, ["mantissaEnd"]: mantissaEnd });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28(units, end, at, exponent, exponentLimit) {
  for (;;) {
    if ((at < end)) {
      const digit = (((units)[at] ?? (-1)) - 48);
      if (((digit < 0) || (digit > 9))) {
        return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_done", ["at"]: at, ["exponent"]: exponent });
      } else {
        const exponent_3 = ((exponent < exponentLimit) ? (() => {
          const exponent_2 = Math.min(exponentLimit, ((exponent * 10) + digit));
          return exponent_2;
        })() : exponent);
        const at_2 = (at + 1);
        [units, end, at, exponent, exponentLimit] = [units, end, at_2, exponent_3, exponentLimit];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_done", ["at"]: at, ["exponent"]: exponent });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23(units, end, at, fraction, digits, significant, dot) {
  for (;;) {
    if ((at < end)) {
      const unit = ((units)[at] ?? (-1));
      if (((unit >= 48) && (unit <= 57))) {
        const digits_2 = (digits + 1);
        const fraction_3 = (dot ? (() => {
          const fraction_2 = (fraction + 1);
          return fraction_2;
        })() : fraction);
        const significant_3 = (((unit !== 48) || (significant > 0)) ? (() => {
          const significant_2 = (significant + 1);
          return significant_2;
        })() : significant);
        const at_2 = (at + 1);
        [units, end, at, fraction, digits, significant, dot] = [units, end, at_2, fraction_3, digits_2, significant_3, dot];
        continue;
      } else {
        if (((unit === 46) && !dot)) {
          const dot_2 = true;
          const at_3 = (at + 1);
          [units, end, at, fraction, digits, significant, dot] = [units, end, at_3, fraction, digits, significant, dot_2];
          continue;
        } else {
          return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_done", ["at"]: at, ["fraction"]: fraction, ["digits"]: digits, ["significant"]: significant });
        }
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_done", ["at"]: at, ["fraction"]: fraction, ["digits"]: digits, ["significant"]: significant });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29(units, end, integerDigits, radix, value, overflow, limit, base, index) {
  for (;;) {
    if ((index < end)) {
      const digit = readStringNumberDigit(((units)[index] ?? (-1)));
      if (((digit < 0) || (digit >= radix))) {
        return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_return", ["ml_source1_temporary_result29"]: NaN });
      } else {
        if (!overflow) {
          const value_2 = ((value * base) + ((integerDigits)[digit] ?? 0n));
          const overflow_2 = (value_2 > limit);
          const index_2 = (index + 1);
          [units, end, integerDigits, radix, value, overflow, limit, base, index] = [units, end, integerDigits, radix, value_2, overflow_2, limit, base, index_2];
          continue;
        } else {
          const index_3 = (index + 1);
          [units, end, integerDigits, radix, value, overflow, limit, base, index] = [units, end, integerDigits, radix, value, overflow, limit, base, index_3];
          continue;
        }
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_done", ["value"]: value, ["overflow"]: overflow });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18(units, index) {
  for (;;) {
    if ((index < units.length)) {
      const unit = ((units)[index] ?? (-1));
      if (((!Number.isSafeInteger(unit) || (unit < 0)) || (unit > 65535))) {
        return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_return", ["ml_source1_temporary_result29"]: NaN });
      } else {
        const index_2 = (index + 1);
        [units, index] = [units, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_done" });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop31(units, start, end) {
  for (;;) {
    if (((start < end) && acceptJavaScriptWhitespaceUnit(((units)[start] ?? (-1))))) {
      const start_2 = (start + 1);
      [units, start, end] = [units, start_2, end];
      continue;
    } else {
      return start;
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop32(units, start, end) {
  for (;;) {
    if (((end > start) && acceptJavaScriptWhitespaceUnit(((units)[(end - 1)] ?? (-1))))) {
      const end_2 = (end - 1);
      [units, start, end] = [units, start, end_2];
      continue;
    } else {
      return end;
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35(units, end, radix, base, digits, value, index) {
  for (;;) {
    if ((index < end)) {
      const digit = readJavaScriptNumericDigitUnit(((units)[index] ?? (-1)));
      if (((digit < 0) || (digit >= radix))) {
        return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_return", ["ml_source1_temporary_result35"]: [] });
      } else {
        const value_2 = ((value * base) + ((digits)[digit] ?? 0n));
        const index_2 = (index + 1);
        [units, end, radix, base, digits, value, index] = [units, end, radix, base, digits, value_2, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_done", ["value"]: value });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30(units, index) {
  for (;;) {
    if ((index < units.length)) {
      const unit = ((units)[index] ?? (-1));
      if (((!Number.isSafeInteger(unit) || (unit < 0)) || (unit > 65535))) {
        return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_return", ["ml_source1_temporary_result35"]: [] });
      } else {
        const index_2 = (index + 1);
        [units, index] = [units, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_done" });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop36(value, integer, weight) {
  for (;;) {
    if ((value >= 1)) {
      const integer_3 = (((value % 2) === 1) ? (() => {
        const integer_2 = (integer + weight);
        return integer_2;
      })() : integer);
      const value_2 = Math.floor((value / 2));
      const weight_2 = (weight * 2n);
      [value, integer, weight] = [value_2, integer_3, weight_2];
      continue;
    } else {
      return integer;
    }
  }
}

function source_ml_source1_temporary_divideJavaScriptBigInteger_loop38(remaining, shifted, weight, quotient) {
  for (;;) {
    if ((weight > 1n)) {
      const shifted_2 = ml_divide(shifted, 2n, 'trunc', "Division by zero", false);
      const weight_2 = ml_divide(weight, 2n, 'trunc', "Division by zero", false);
      if ((shifted_2 <= remaining)) {
        const remaining_2 = (remaining - shifted_2);
        const quotient_2 = (quotient + weight_2);
        [remaining, shifted, weight, quotient] = [remaining_2, shifted_2, weight_2, quotient_2];
        continue;
      } else {
        [remaining, shifted, weight, quotient] = [remaining, shifted_2, weight_2, quotient];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_divideJavaScriptBigInteger_loop38_done", ["remaining"]: remaining, ["quotient"]: quotient });
    }
  }
}

function source_ml_source1_temporary_divideJavaScriptBigInteger_loop37(remaining, shifted, weight) {
  for (;;) {
    if ((shifted <= remaining)) {
      const shifted_2 = (shifted * 2n);
      const weight_2 = (weight * 2n);
      [remaining, shifted, weight] = [remaining, shifted_2, weight_2];
      continue;
    } else {
      return ({ $: "ml_source1_temporary_divideJavaScriptBigInteger_loop37_done", ["shifted"]: shifted, ["weight"]: weight });
    }
  }
}

function source_ml_source1_temporary_powerJavaScriptBigInteger_loop39(result, weight, remaining) {
  for (;;) {
    if ((remaining > 0n)) {
      const result_3 = ((ml_divide(remaining, 2n, 'trunc', "Division by zero", true) === 1n) ? (() => {
        const result_2 = (result * weight);
        return result_2;
      })() : result);
      const remaining_2 = ml_divide(remaining, 2n, 'trunc', "Division by zero", false);
      if ((remaining_2 > 0n)) {
        const weight_2 = (weight * weight);
        [result, weight, remaining] = [result_3, weight_2, remaining_2];
        continue;
      } else {
        [result, weight, remaining] = [result_3, weight, remaining_2];
        continue;
      }
    } else {
      return result;
    }
  }
}

function source_ml_source1_temporary_normalizeNumberPowerBound_loop41(value, exponent, scale) {
  for (;;) {
    if ((value < scale)) {
      const value_2 = (value * 2n);
      const exponent_2 = (exponent - 1n);
      [value, exponent, scale] = [value_2, exponent_2, scale];
      continue;
    } else {
      return ({ $: "ml_source1_temporary_normalizeNumberPowerBound_loop41_done", ["value"]: value, ["exponent"]: exponent });
    }
  }
}

function source_ml_source1_temporary_normalizeNumberPowerBound_loop40(value, exponent, scale, upward) {
  for (;;) {
    if ((value >= (scale * 2n))) {
      const value_2 = ml_divide((value + (upward ? 1n : 0n)), 2n, 'trunc', "Division by zero", false);
      const exponent_2 = (exponent + 1n);
      [value, exponent, scale, upward] = [value_2, exponent_2, scale, upward];
      continue;
    } else {
      return ({ $: "ml_source1_temporary_normalizeNumberPowerBound_loop40_done", ["value"]: value, ["exponent"]: exponent });
    }
  }
}

function source_ml_source1_temporary_rootNumberPowerInteger_loop42(root, remaining) {
  for (;;) {
    if ((remaining > 1n)) {
      const remaining_2 = ml_divide(remaining, 4n, 'trunc', "Division by zero", false);
      const root_2 = (root * 2n);
      [root, remaining] = [root_2, remaining_2];
      continue;
    } else {
      return root;
    }
  }
}

function source_ml_source1_temporary_rootNumberPowerInteger_loop43(value, root) {
  for (;;) {
    const next = ml_divide((root + divideNumberPowerBound(value, root, false)), 2n, 'trunc', "Division by zero", false);
    if ((next >= root)) {
      return root;
    } else {
      const root_2 = next;
      [value, root] = [value, root_2];
      continue;
    }
  }
}

function source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46(value, exponent) {
  for (;;) {
    if ((value < 1)) {
      const value_2 = (value * 2);
      const exponent_2 = (exponent - 1n);
      [value, exponent] = [value_2, exponent_2];
      continue;
    } else {
      return ({ $: "ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_done", ["value"]: value, ["exponent"]: exponent });
    }
  }
}

function source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45(value, exponent) {
  for (;;) {
    if ((value >= 2)) {
      const value_2 = (value / 2);
      const exponent_2 = (exponent + 1n);
      [value, exponent] = [value_2, exponent_2];
      continue;
    } else {
      return ({ $: "ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_done", ["value"]: value, ["exponent"]: exponent });
    }
  }
}

function source_ml_source1_temporary_powerJavaScriptNumber_loop49(scale, interval, shift) {
  for (;;) {
    if ((shift < 0n)) {
      const interval_2 = rootNumberPowerInterval(interval, scale);
      const shift_2 = (shift + 1n);
      [scale, interval, shift] = [scale, interval_2, shift_2];
      continue;
    } else {
      return interval;
    }
  }
}

function source_ml_source1_temporary_powerJavaScriptNumber_loop50(scale, interval, result, remaining) {
  for (;;) {
    if ((remaining > 0n)) {
      const result_3 = ((ml_divide(remaining, 2n, 'trunc', "Division by zero", true) === 1n) ? (() => {
        const result_2 = multiplyNumberPowerInterval(result, interval, scale);
        return result_2;
      })() : result);
      const remaining_2 = ml_divide(remaining, 2n, 'trunc', "Division by zero", false);
      if ((remaining_2 > 0n)) {
        const interval_2 = multiplyNumberPowerInterval(interval, interval, scale);
        [scale, interval, result, remaining] = [scale, interval_2, result_3, remaining_2];
        continue;
      } else {
        [scale, interval, result, remaining] = [scale, interval, result_3, remaining_2];
        continue;
      }
    } else {
      return result;
    }
  }
}

function source_ml_source1_temporary_powerJavaScriptNumber_loop48(scale, interval, shift) {
  for (;;) {
    if ((shift > 0n)) {
      const interval_2 = multiplyNumberPowerInterval(interval, interval, scale);
      const shift_2 = (shift - 1n);
      [scale, interval, shift] = [scale, interval_2, shift_2];
      continue;
    } else {
      return ({ $: "ml_source1_temporary_powerJavaScriptNumber_loop48_done", ["interval"]: interval, ["shift"]: shift });
    }
  }
}

function source_ml_source1_temporary_powerJavaScriptNumber_loop47(exponent, negative, input, power, precision) {
  for (;;) {
    const scale = ((powerJavaScriptBigInteger(2n, precision))[0] ?? 0n);
    const value = (((input)[0] ?? 0n) * ((powerJavaScriptBigInteger(2n, (precision - 52n)))[0] ?? 0n));
    const initialExponent = (((input)[1] ?? 0n) + 52n);
    const interval = [value, initialExponent, value, initialExponent];
    const interval_3 = ((exponent < 0) ? (() => {
      const interval_2 = invertNumberPowerInterval(interval, scale);
      return interval_2;
    })() : interval);
    const shift = ((power)[1] ?? 0n);
    const source_ml_source2_s38 = source_ml_source1_temporary_powerJavaScriptNumber_loop48(scale, interval_3, shift);
    switch (source_ml_source2_s38.$) {
      case "ml_source1_temporary_powerJavaScriptNumber_loop48_done": {
        const interval_4 = source_ml_source2_s38["interval"];
        const shift_2 = source_ml_source2_s38["shift"];
        const interval_5 = source_ml_source1_temporary_powerJavaScriptNumber_loop49(scale, interval_4, shift_2);
        const result = [scale, 0n, scale, 0n];
        const remaining = ((power)[0] ?? 0n);
        const result_2 = source_ml_source1_temporary_powerJavaScriptNumber_loop50(scale, interval_5, result, remaining);
        const lower = roundNumberPowerBound(((result_2)[0] ?? 0n), ((result_2)[1] ?? 0n), precision);
        const upper = roundNumberPowerBound(((result_2)[2] ?? 0n), ((result_2)[3] ?? 0n), precision);
        if ((lower === upper)) {
          if (negative) {
            return -lower;
          } else {
            return lower;
          }
        } else {
          const precision_2 = (precision * 2n);
          [exponent, negative, input, power, precision] = [exponent, negative, input, power, precision_2];
          continue;
        }
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source2_s38.$}`);
      }
    }
  }
}

function main() {

}

main();

return { convertUnits: convertJavaScriptStringBigIntegerUnits, convertNumber: convertJavaScriptNumberBigInteger, convertBoolean: convertJavaScriptBooleanBigInteger, convertUtf8: convertJavaScriptUtf8BigInteger, divide: divideJavaScriptBigInteger, power: powerJavaScriptBigInteger, powerNumber: powerJavaScriptNumber, compareNumber: compareJavaScriptNumberBigInteger, compareUnits: compareJavaScriptStringBigIntegerUnits, compareUtf8: compareJavaScriptUtf8BigInteger };
})();
