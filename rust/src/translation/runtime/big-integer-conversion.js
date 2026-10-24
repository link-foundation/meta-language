const ml_big_integer_conversion = (() => {
// Translated from JavaScript by meta-language: portable core, JavaScript target.
'use strict';

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

function convertJavaScriptStringBigIntegerUnits(units) {
  const index = 0;
  const source_ml_source2_s3 = source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5(units, index);
  switch (source_ml_source2_s3.$) {
    case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_done": {
      const start = 0;
      const end = units.length;
      const start_2 = source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop6(units, start, end);
      const end_2 = source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop7(units, start_2, end);
      if ((start_2 === end_2)) {
        return [0n];
      } else {
        const negative = false;
        const signed = false;
        const sign = ((units)[start_2] ?? (-1));
        const source_ml_source2_s4 = (((sign === 43) || (sign === 45)) ? (() => {
          const negative_2 = (sign === 45);
          const signed_2 = true;
          const start_3 = (start_2 + 1);
          return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8_next", ["start"]: start_3, ["negative"]: negative_2, ["signed"]: signed_2 });
        })() : ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8_next", ["start"]: start_2, ["negative"]: negative, ["signed"]: signed }));
        switch (source_ml_source2_s4.$) {
          case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8_next": {
            const start_4 = source_ml_source2_s4["start"];
            const negative_3 = source_ml_source2_s4["negative"];
            const signed_3 = source_ml_source2_s4["signed"];
            if ((start_4 === end_2)) {
              return [];
            } else {
              const radix = 10;
              const base = 10n;
              const source_ml_source2_s5 = (((!signed_3 && (((units)[start_4] ?? (-1)) === 48)) && ((end_2 - start_4) >= 2)) ? (() => {
                const marker = ((units)[(start_4 + 1)] ?? (-1));
                const selected = (((marker === 120) || (marker === 88)) ? 16 : (((marker === 98) || (marker === 66)) ? 2 : (((marker === 111) || (marker === 79)) ? 8 : 0)));
                if ((selected !== 0)) {
                  const radix_2 = selected;
                  const base_2 = ((selected === 16) ? 16n : ((selected === 8) ? 8n : 2n));
                  const start_5 = (start_4 + 2);
                  return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next", ["start"]: start_5, ["radix"]: radix_2, ["base"]: base_2 });
                } else {
                  return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next", ["start"]: start_4, ["radix"]: radix, ["base"]: base });
                }
              })() : ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next", ["start"]: start_4, ["radix"]: radix, ["base"]: base }));
              switch (source_ml_source2_s5.$) {
                case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next": {
                  const start_6 = source_ml_source2_s5["start"];
                  const radix_3 = source_ml_source2_s5["radix"];
                  const base_3 = source_ml_source2_s5["base"];
                  if ((start_6 === end_2)) {
                    return [];
                  } else {
                    const digits = [0n, 1n, 2n, 3n, 4n, 5n, 6n, 7n, 8n, 9n, 10n, 11n, 12n, 13n, 14n, 15n];
                    const value = 0n;
                    const index_2 = start_6;
                    const source_ml_source2_s6 = source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10(units, end_2, radix_3, base_3, digits, value, index_2);
                    switch (source_ml_source2_s6.$) {
                      case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_done": {
                        const value_2 = source_ml_source2_s6["value"];
                        return [(negative_3 ? -value_2 : value_2)];
                      }
                      case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_return": {
                        const source_ml_source1_temporary_result10 = source_ml_source2_s6["ml_source1_temporary_result10"];
                        return source_ml_source1_temporary_result10;
                      }
                      default: {
                        throw new TypeError(`unexpected constructor ${source_ml_source2_s6.$}`);
                      }
                    }
                  }
                }
                default: {
                  throw new TypeError(`unexpected constructor ${source_ml_source2_s5.$}`);
                }
              }
            }
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source2_s4.$}`);
          }
        }
      }
    }
    case "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_return": {
      const source_ml_source1_temporary_result10_2 = source_ml_source2_s3["ml_source1_temporary_result10"];
      return source_ml_source1_temporary_result10_2;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source2_s3.$}`);
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
    const integer_2 = source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop11(value_2, integer, weight);
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

function acceptLinkedJavaScriptBigIntegerResult(kind, operation) {
  return ((kind === "math") && (readJavaScriptBigIntegerResultOperation(operation) !== ""));
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
      const source_ml_source2_s7 = (((scalar >= 194) && (scalar <= 223)) ? (() => {
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
      switch (source_ml_source2_s7.$) {
        case "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next": {
          const scalar_5 = source_ml_source2_s7["scalar"];
          const remaining_5 = source_ml_source2_s7["remaining"];
          const minimum_5 = source_ml_source2_s7["minimum"];
          const source_ml_source2_s8 = source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4(bytes, index_2, scalar_5, remaining_5);
          switch (source_ml_source2_s8.$) {
            case "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_done": {
              const index_3 = source_ml_source2_s8["index"];
              const scalar_6 = source_ml_source2_s8["scalar"];
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
              const source_ml_source1_temporary_result4 = source_ml_source2_s8["ml_source1_temporary_result4"];
              return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_source1_temporary_result2"]: source_ml_source1_temporary_result4 });
            }
            default: {
              throw new TypeError(`unexpected constructor ${source_ml_source2_s8.$}`);
            }
          }
        }
        case "ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_return": {
          const source_ml_source1_temporary_result4_2 = source_ml_source2_s7["ml_source1_temporary_result4"];
          return ({ $: "ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_source1_temporary_result2"]: source_ml_source1_temporary_result4_2 });
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source2_s7.$}`);
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

function source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop6(units, start, end) {
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

function source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop7(units, start, end) {
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

function source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10(units, end, radix, base, digits, value, index) {
  for (;;) {
    if ((index < end)) {
      const digit = readJavaScriptNumericDigitUnit(((units)[index] ?? (-1)));
      if (((digit < 0) || (digit >= radix))) {
        return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_return", ["ml_source1_temporary_result10"]: [] });
      } else {
        const value_2 = ((value * base) + ((digits)[digit] ?? 0n));
        const index_2 = (index + 1);
        [units, end, radix, base, digits, value, index] = [units, end, radix, base, digits, value_2, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_done", ["value"]: value });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5(units, index) {
  for (;;) {
    if ((index < units.length)) {
      const unit = ((units)[index] ?? (-1));
      if (((!Number.isSafeInteger(unit) || (unit < 0)) || (unit > 65535))) {
        return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_return", ["ml_source1_temporary_result10"]: [] });
      } else {
        const index_2 = (index + 1);
        [units, index] = [units, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_done" });
    }
  }
}

function source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop11(value, integer, weight) {
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

function main() {

}

main();

return { convertUnits: convertJavaScriptStringBigIntegerUnits, convertNumber: convertJavaScriptNumberBigInteger, convertBoolean: convertJavaScriptBooleanBigInteger, convertUtf8: convertJavaScriptUtf8BigInteger };
})();
