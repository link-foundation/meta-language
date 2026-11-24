const ml_utf16_text = (() => {
// Translated from JavaScript by meta-language: portable core, JavaScript target.
'use strict';

function acceptJavaScriptWhitespaceUnit(unit) {
  return ((((((((((((unit >= 9) && (unit <= 13)) || (unit === 32)) || (unit === 160)) || (unit === 5760)) || ((unit >= 8192) && (unit <= 8202))) || (unit === 8232)) || (unit === 8233)) || (unit === 8239)) || (unit === 8287)) || (unit === 12288)) || (unit === 65279));
}

function readJavaScriptUtf8StringUnits(bytes) {
  const index = 0;
  const source_ml_source1_s1 = source_ml_temporary_readJavaScriptUtf8StringUnits_loop1(bytes, index);
  switch (source_ml_source1_s1.$) {
    case "ml_temporary_readJavaScriptUtf8StringUnits_loop1_done": {
      const units = [];
      const index_2 = 0;
      const source_ml_source1_s2 = source_ml_temporary_readJavaScriptUtf8StringUnits_loop2(bytes, units, index_2);
      switch (source_ml_source1_s2.$) {
        case "ml_temporary_readJavaScriptUtf8StringUnits_loop2_done": {
          const units_2 = source_ml_source1_s2["units"];
          return units_2;
        }
        case "ml_temporary_readJavaScriptUtf8StringUnits_loop2_return": {
          const source_ml_temporary_result2 = source_ml_source1_s2["ml_temporary_result2"];
          return source_ml_temporary_result2;
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s2.$}`);
        }
      }
    }
    case "ml_temporary_readJavaScriptUtf8StringUnits_loop1_return": {
      const source_ml_temporary_result4 = source_ml_source1_s1["ml_temporary_result4"];
      return source_ml_temporary_result4;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s1.$}`);
    }
  }
}

function normalizeJavaScriptUnsignedInteger(value) {
  if ((!Number.isFinite(value) || (value === 0))) {
    return 0;
  } else {
    const integral = ((value < 0) ? -Math.floor(-value) : Math.floor(value));
    const remainder = (integral % 4294967296);
    if ((remainder < 0)) {
      return (remainder + 4294967296);
    } else {
      if ((remainder === 0)) {
        return 0;
      } else {
        return remainder;
      }
    }
  }
}

function readUtf16TextInteger(value) {
  if (Number.isNaN(value)) {
    return 0;
  } else {
    return Math.trunc(value);
  }
}

function readUtf16TextSlicePosition(value, length) {
  const integer = readUtf16TextInteger(value);
  if ((integer < 0)) {
    return Math.max((length + integer), 0);
  } else {
    return Math.min(integer, length);
  }
}

function sliceUtf16Text(text, start, end) {
  const first = readUtf16TextSlicePosition(start, text.length);
  const last = readUtf16TextSlicePosition(end, text.length);
  const result = [];
  const at = first;
  const result_2 = source_ml_temporary_sliceUtf16Text_loop5(text, last, result, at);
  return result_2;
}

function substringUtf16Text(text, start, end) {
  const first = Math.min(Math.max(readUtf16TextInteger(start), 0), text.length);
  const last = Math.min(Math.max(readUtf16TextInteger(end), 0), text.length);
  return sliceUtf16Text(text, Math.min(first, last), Math.max(first, last));
}

function characterUtf16Text(text, position) {
  const at = readUtf16TextInteger(position);
  if (((at < 0) || (at >= text.length))) {
    return [];
  } else {
    return [((text)[at] ?? 0)];
  }
}

function relativeCharacterUtf16Text(text, position) {
  const integer = readUtf16TextInteger(position);
  const at = ((integer < 0) ? (text.length + integer) : integer);
  if (((at < 0) || (at >= text.length))) {
    return [];
  } else {
    return [[((text)[at] ?? 0)]];
  }
}

function codePointUtf16Text(text, position) {
  const at = readUtf16TextInteger(position);
  if (((at < 0) || (at >= text.length))) {
    return [];
  } else {
    const first = ((text)[at] ?? 0);
    const second = ((text)[(at + 1)] ?? (-1));
    return [(((((first >= 55296) && (first <= 56319)) && (second >= 56320)) && (second <= 57343)) ? (((65536 + ((first - 55296) * 1024)) + second) - 56320) : first)];
  }
}

function pointsUtf16Text(text) {
  const result = [];
  const at = 0;
  const result_2 = source_ml_temporary_pointsUtf16Text_loop6(text, result, at);
  return result_2;
}

function splitUtf16Text(text, separator, limit, missing) {
  const count = normalizeJavaScriptUnsignedInteger(limit);
  const result = [];
  if ((count === 0)) {
    return result;
  } else {
    if (missing) {
      return [sliceUtf16Text(text, 0, text.length)];
    } else {
      if ((separator.length === 0)) {
        const at = 0;
        const result_2 = source_ml_temporary_splitUtf16Text_loop7(text, count, result, at);
        return result_2;
      } else {
        const start = 0;
        const source_ml_source1_s3 = source_ml_temporary_splitUtf16Text_loop8(text, separator, count, result, start);
        switch (source_ml_source1_s3.$) {
          case "ml_temporary_splitUtf16Text_loop8_done": {
            const result_3 = source_ml_source1_s3["result"];
            const start_2 = source_ml_source1_s3["start"];
            const result_4 = [...result_3, ...[sliceUtf16Text(text, start_2, text.length)]];
            return result_4;
          }
          case "ml_temporary_splitUtf16Text_loop8_return": {
            const source_ml_temporary_result8 = source_ml_source1_s3["ml_temporary_result8"];
            return source_ml_temporary_result8;
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source1_s3.$}`);
          }
        }
      }
    }
  }
}

function matchUtf16Text(text, search, start) {
  if (((start < 0) || ((start + search.length) > text.length))) {
    return false;
  } else {
    const at = 0;
    const source_ml_source1_s4 = source_ml_temporary_matchUtf16Text_loop9(text, search, start, at);
    switch (source_ml_source1_s4.$) {
      case "ml_temporary_matchUtf16Text_loop9_done": {
        return true;
      }
      case "ml_temporary_matchUtf16Text_loop9_return": {
        const source_ml_temporary_result9 = source_ml_source1_s4["ml_temporary_result9"];
        return source_ml_temporary_result9;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s4.$}`);
      }
    }
  }
}

function indexUtf16Text(text, search, position) {
  const start = Math.min(Math.max(readUtf16TextInteger(position), 0), text.length);
  const at = start;
  const source_ml_source1_s5 = source_ml_temporary_indexUtf16Text_loop10(text, search, at);
  switch (source_ml_source1_s5.$) {
    case "ml_temporary_indexUtf16Text_loop10_done": {
      return (-1);
    }
    case "ml_temporary_indexUtf16Text_loop10_return": {
      const source_ml_temporary_result10 = source_ml_source1_s5["ml_temporary_result10"];
      return source_ml_temporary_result10;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s5.$}`);
    }
  }
}

function lastIndexUtf16Text(text, search, position) {
  const integer = (Number.isNaN(position) ? Infinity : readUtf16TextInteger(position));
  const start = Math.min(Math.max(integer, 0), (text.length - search.length));
  const at = start;
  const source_ml_source1_s6 = source_ml_temporary_lastIndexUtf16Text_loop11(text, search, at);
  switch (source_ml_source1_s6.$) {
    case "ml_temporary_lastIndexUtf16Text_loop11_done": {
      return (-1);
    }
    case "ml_temporary_lastIndexUtf16Text_loop11_return": {
      const source_ml_temporary_result11 = source_ml_source1_s6["ml_temporary_result11"];
      return source_ml_temporary_result11;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s6.$}`);
    }
  }
}

function compareUtf16Text(left, right) {
  const count = Math.min(left.length, right.length);
  const at = 0;
  const source_ml_source1_s7 = source_ml_temporary_compareUtf16Text_loop12(left, right, count, at);
  switch (source_ml_source1_s7.$) {
    case "ml_temporary_compareUtf16Text_loop12_done": {
      if ((left.length === right.length)) {
        return 0;
      } else {
        if ((left.length < right.length)) {
          return (-1);
        } else {
          return 1;
        }
      }
    }
    case "ml_temporary_compareUtf16Text_loop12_return": {
      const source_ml_temporary_result12 = source_ml_source1_s7["ml_temporary_result12"];
      return source_ml_temporary_result12;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s7.$}`);
    }
  }
}

function startsUtf16Text(text, search) {
  return matchUtf16Text(text, search, 0);
}

function endsUtf16Text(text, search) {
  return matchUtf16Text(text, search, (text.length - search.length));
}

function includesUtf16Text(text, search) {
  return (indexUtf16Text(text, search, 0) >= 0);
}

function joinUtf16Text(values, separator) {
  const result = [];
  const at = 0;
  const result_2 = source_ml_temporary_joinUtf16Text_loop13(values, separator, result, at);
  return result_2;
}

function trimUtf16Text(text, start, end) {
  const first = 0;
  const last = text.length;
  const first_3 = (start ? (() => {
    const first_2 = source_ml_temporary_trimUtf16Text_loop16(text, first, last);
    return first_2;
  })() : first);
  const last_3 = (end ? (() => {
    const last_2 = source_ml_temporary_trimUtf16Text_loop17(text, first_3, last);
    return last_2;
  })() : last);
  return sliceUtf16Text(text, first_3, last_3);
}

function characterUnitsUtf16Text(values) {
  const result = [];
  const at = 0;
  const result_2 = source_ml_temporary_characterUnitsUtf16Text_loop18(values, result, at);
  return result_2;
}

function codePointUnitsUtf16Text(values) {
  const result = [(-1)];
  const at = 0;
  const source_ml_source1_s8 = source_ml_temporary_codePointUnitsUtf16Text_loop19(values, result, at);
  switch (source_ml_source1_s8.$) {
    case "ml_temporary_codePointUnitsUtf16Text_loop19_done": {
      const result_2 = source_ml_source1_s8["result"];
      return result_2;
    }
    case "ml_temporary_codePointUnitsUtf16Text_loop19_return": {
      const source_ml_temporary_result19 = source_ml_source1_s8["ml_temporary_result19"];
      return source_ml_temporary_result19;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s8.$}`);
    }
  }
}

function readUtf8TextUnits(bytes) {
  return readJavaScriptUtf8StringUnits(bytes);
}

function displayPointsUtf16Text(text) {
  const result = [];
  const at = 0;
  const result_2 = source_ml_temporary_displayPointsUtf16Text_loop20(text, result, at);
  return result_2;
}

function displayUtf8FromUtf16Text(text) {
  const bytes = [];
  const points = displayPointsUtf16Text(text);
  const at = 0;
  const bytes_2 = source_ml_temporary_displayUtf8FromUtf16Text_loop22(bytes, points, at);
  return bytes_2;
}

function planUtf16TextSubstitution(replacement) {
  const result = [];
  const start = 0;
  const at = 0;
  const source_ml_source1_s9 = source_ml_temporary_planUtf16TextSubstitution_loop23(replacement, result, start, at);
  switch (source_ml_source1_s9.$) {
    case "ml_temporary_planUtf16TextSubstitution_loop23_done": {
      const result_2 = source_ml_source1_s9["result"];
      const start_2 = source_ml_source1_s9["start"];
      const result_4 = ((start_2 < replacement.length) ? (() => {
        const result_3 = [...result_2, ...[0, start_2, replacement.length]];
        return result_3;
      })() : result_2);
      return result_4;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s9.$}`);
    }
  }
}

function replaceUtf16Text(text, search, replacement, all) {
  const result = [];
  const plan = planUtf16TextSubstitution(replacement);
  const start = 0;
  const from = 0;
  const searching = true;
  const source_ml_source1_s10 = source_ml_temporary_replaceUtf16Text_loop24(text, search, replacement, all, result, plan, start, from, searching);
  switch (source_ml_source1_s10.$) {
    case "ml_temporary_replaceUtf16Text_loop24_done": {
      const result_2 = source_ml_source1_s10["result"];
      const start_2 = source_ml_source1_s10["start"];
      const at = start_2;
      const result_3 = source_ml_temporary_replaceUtf16Text_loop25(text, result_2, at);
      return result_3;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s10.$}`);
    }
  }
}

function source_ml_temporary_readJavaScriptUtf8StringUnits_loop4(bytes, index, scalar, remaining) {
  for (;;) {
    if ((remaining > 0)) {
      const next = ((bytes)[index] ?? (-1));
      if (((next < 128) || (next > 191))) {
        return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop4_return", ["ml_temporary_result4"]: [(-1)] });
      } else {
        const scalar_2 = (((scalar * 64) + next) - 128);
        const index_2 = (index + 1);
        const remaining_2 = (remaining - 1);
        [bytes, index, scalar, remaining] = [bytes, index_2, scalar_2, remaining_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop4_done", ["index"]: index, ["scalar"]: scalar });
    }
  }
}

function source_ml_temporary_readJavaScriptUtf8StringUnits_loop2(bytes, units, index) {
  for (;;) {
    if ((index < bytes.length)) {
      const scalar = ((bytes)[index] ?? (-1));
      const index_2 = (index + 1);
      const remaining = 0;
      const minimum = 0;
      const source_ml_source1_s11 = (((scalar >= 194) && (scalar <= 223)) ? (() => {
        const scalar_2 = (scalar - 192);
        const remaining_2 = 1;
        const minimum_2 = 128;
        return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_join3_next", ["scalar"]: scalar_2, ["remaining"]: remaining_2, ["minimum"]: minimum_2 });
      })() : (((scalar >= 224) && (scalar <= 239)) ? (() => {
        const scalar_3 = (scalar - 224);
        const remaining_3 = 2;
        const minimum_3 = 2048;
        return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_join3_next", ["scalar"]: scalar_3, ["remaining"]: remaining_3, ["minimum"]: minimum_3 });
      })() : (((scalar >= 240) && (scalar <= 244)) ? (() => {
        const scalar_4 = (scalar - 240);
        const remaining_4 = 3;
        const minimum_4 = 65536;
        return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_join3_next", ["scalar"]: scalar_4, ["remaining"]: remaining_4, ["minimum"]: minimum_4 });
      })() : ((scalar >= 128) ? ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_join3_return", ["ml_temporary_result4"]: [(-1)] }) : ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_join3_next", ["scalar"]: scalar, ["remaining"]: remaining, ["minimum"]: minimum })))));
      switch (source_ml_source1_s11.$) {
        case "ml_temporary_readJavaScriptUtf8StringUnits_join3_next": {
          const scalar_5 = source_ml_source1_s11["scalar"];
          const remaining_5 = source_ml_source1_s11["remaining"];
          const minimum_5 = source_ml_source1_s11["minimum"];
          const source_ml_source1_s12 = source_ml_temporary_readJavaScriptUtf8StringUnits_loop4(bytes, index_2, scalar_5, remaining_5);
          switch (source_ml_source1_s12.$) {
            case "ml_temporary_readJavaScriptUtf8StringUnits_loop4_done": {
              const index_3 = source_ml_source1_s12["index"];
              const scalar_6 = source_ml_source1_s12["scalar"];
              if ((((scalar_6 < minimum_5) || (scalar_6 > 1114111)) || ((scalar_6 >= 55296) && (scalar_6 <= 57343)))) {
                return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_temporary_result2"]: [(-1)] });
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
            case "ml_temporary_readJavaScriptUtf8StringUnits_loop4_return": {
              const source_ml_temporary_result4 = source_ml_source1_s12["ml_temporary_result4"];
              return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_temporary_result2"]: source_ml_temporary_result4 });
            }
            default: {
              throw new TypeError(`unexpected constructor ${source_ml_source1_s12.$}`);
            }
          }
        }
        case "ml_temporary_readJavaScriptUtf8StringUnits_join3_return": {
          const source_ml_temporary_result4_2 = source_ml_source1_s11["ml_temporary_result4"];
          return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_temporary_result2"]: source_ml_temporary_result4_2 });
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s11.$}`);
        }
      }
    } else {
      return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop2_done", ["units"]: units });
    }
  }
}

function source_ml_temporary_readJavaScriptUtf8StringUnits_loop1(bytes, index) {
  for (;;) {
    if ((index < bytes.length)) {
      const unit = ((bytes)[index] ?? (-1));
      if (((!Number.isSafeInteger(unit) || (unit < 0)) || (unit > 255))) {
        return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop1_return", ["ml_temporary_result4"]: [(-1)] });
      } else {
        const index_2 = (index + 1);
        [bytes, index] = [bytes, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop1_done" });
    }
  }
}

function source_ml_temporary_sliceUtf16Text_loop5(text, last, result, at) {
  for (;;) {
    if ((at < last)) {
      const result_2 = [...result, ...[((text)[at] ?? 0)]];
      const at_2 = (at + 1);
      [text, last, result, at] = [text, last, result_2, at_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_pointsUtf16Text_loop6(text, result, at) {
  for (;;) {
    if ((at < text.length)) {
      const first = ((text)[at] ?? 0);
      const second = ((text)[(at + 1)] ?? (-1));
      if (((((first >= 55296) && (first <= 56319)) && (second >= 56320)) && (second <= 57343))) {
        const result_2 = [...result, ...[[first, second]]];
        const at_2 = (at + 1);
        const at_3 = (at_2 + 1);
        [text, result, at] = [text, result_2, at_3];
        continue;
      } else {
        const result_3 = [...result, ...[[first]]];
        const at_4 = (at + 1);
        [text, result, at] = [text, result_3, at_4];
        continue;
      }
    } else {
      return result;
    }
  }
}

function source_ml_temporary_splitUtf16Text_loop7(text, count, result, at) {
  for (;;) {
    if (((at < text.length) && (result.length < count))) {
      const result_2 = [...result, ...[[((text)[at] ?? 0)]]];
      const at_2 = (at + 1);
      [text, count, result, at] = [text, count, result_2, at_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_splitUtf16Text_loop8(text, separator, count, result, start) {
  for (;;) {
    const found = indexUtf16Text(text, separator, start);
    if ((found < 0)) {
      return ({ $: "ml_temporary_splitUtf16Text_loop8_done", ["result"]: result, ["start"]: start });
    } else {
      const result_2 = [...result, ...[sliceUtf16Text(text, start, found)]];
      if ((result_2.length === count)) {
        return ({ $: "ml_temporary_splitUtf16Text_loop8_return", ["ml_temporary_result8"]: result_2 });
      } else {
        const start_2 = (found + separator.length);
        [text, separator, count, result, start] = [text, separator, count, result_2, start_2];
        continue;
      }
    }
  }
}

function source_ml_temporary_matchUtf16Text_loop9(text, search, start, at) {
  for (;;) {
    if ((at < search.length)) {
      if ((() => {
        const source_ml_source1_comparison_operand13 = (text)[(start + at)];
        const source_ml_source1_comparison_operand14 = (search)[at];
        return (source_ml_source1_comparison_operand13 !== source_ml_source1_comparison_operand14);
      })()) {
        return ({ $: "ml_temporary_matchUtf16Text_loop9_return", ["ml_temporary_result9"]: false });
      } else {
        const at_2 = (at + 1);
        [text, search, start, at] = [text, search, start, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_matchUtf16Text_loop9_done" });
    }
  }
}

function source_ml_temporary_indexUtf16Text_loop10(text, search, at) {
  for (;;) {
    if ((at <= (text.length - search.length))) {
      if (matchUtf16Text(text, search, at)) {
        return ({ $: "ml_temporary_indexUtf16Text_loop10_return", ["ml_temporary_result10"]: at });
      } else {
        const at_2 = (at + 1);
        [text, search, at] = [text, search, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_indexUtf16Text_loop10_done" });
    }
  }
}

function source_ml_temporary_lastIndexUtf16Text_loop11(text, search, at) {
  for (;;) {
    if ((at >= 0)) {
      if (matchUtf16Text(text, search, at)) {
        return ({ $: "ml_temporary_lastIndexUtf16Text_loop11_return", ["ml_temporary_result11"]: at });
      } else {
        const at_2 = (at - 1);
        [text, search, at] = [text, search, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_lastIndexUtf16Text_loop11_done" });
    }
  }
}

function source_ml_temporary_compareUtf16Text_loop12(left, right, count, at) {
  for (;;) {
    if ((at < count)) {
      const a = ((left)[at] ?? 0);
      const b = ((right)[at] ?? 0);
      if ((a !== b)) {
        return ({ $: "ml_temporary_compareUtf16Text_loop12_return", ["ml_temporary_result12"]: ((a < b) ? (-1) : 1) });
      } else {
        const at_2 = (at + 1);
        [left, right, count, at] = [left, right, count, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_compareUtf16Text_loop12_done" });
    }
  }
}

function source_ml_temporary_joinUtf16Text_loop14(separator, result, offset) {
  for (;;) {
    if ((offset < separator.length)) {
      const result_2 = [...result, ...[((separator)[offset] ?? 0)]];
      const offset_2 = (offset + 1);
      [separator, result, offset] = [separator, result_2, offset_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_joinUtf16Text_loop15(result, value, offset) {
  for (;;) {
    if ((offset < value.length)) {
      const result_2 = [...result, ...[((value)[offset] ?? 0)]];
      const offset_2 = (offset + 1);
      [result, value, offset] = [result_2, value, offset_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_joinUtf16Text_loop13(values, separator, result, at) {
  for (;;) {
    if ((at < values.length)) {
      const result_3 = ((at !== 0) ? (() => {
        const offset = 0;
        const result_2 = source_ml_temporary_joinUtf16Text_loop14(separator, result, offset);
        return result_2;
      })() : result);
      const value = ((values)[at] ?? []);
      const offset_2 = 0;
      const result_4 = source_ml_temporary_joinUtf16Text_loop15(result_3, value, offset_2);
      const at_2 = (at + 1);
      [values, separator, result, at] = [values, separator, result_4, at_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_trimUtf16Text_loop16(text, first, last) {
  for (;;) {
    if (((first < last) && acceptJavaScriptWhitespaceUnit(((text)[first] ?? (-1))))) {
      const first_2 = (first + 1);
      [text, first, last] = [text, first_2, last];
      continue;
    } else {
      return first;
    }
  }
}

function source_ml_temporary_trimUtf16Text_loop17(text, first, last) {
  for (;;) {
    if (((last > first) && acceptJavaScriptWhitespaceUnit(((text)[(last - 1)] ?? (-1))))) {
      const last_2 = (last - 1);
      [text, first, last] = [text, first, last_2];
      continue;
    } else {
      return last;
    }
  }
}

function source_ml_temporary_characterUnitsUtf16Text_loop18(values, result, at) {
  for (;;) {
    if ((at < values.length)) {
      const value = ((values)[at] ?? 0);
      const integer = (Number.isFinite(value) ? Math.trunc(value) : 0);
      const residue = (integer % 65536);
      const result_2 = [...result, ...[((residue < 0) ? (residue + 65536) : ((residue === 0) ? 0 : residue))]];
      const at_2 = (at + 1);
      [values, result, at] = [values, result_2, at_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_codePointUnitsUtf16Text_loop19(values, result, at) {
  for (;;) {
    if ((at < values.length)) {
      const value = ((values)[at] ?? (-1));
      if (((!Number.isInteger(value) || (value < 0)) || (value > 1114111))) {
        return ({ $: "ml_temporary_codePointUnitsUtf16Text_loop19_return", ["ml_temporary_result19"]: [at] });
      } else {
        if ((value < 65536)) {
          const result_2 = [...result, ...[((value === 0) ? 0 : value)]];
          const at_2 = (at + 1);
          [values, result, at] = [values, result_2, at_2];
          continue;
        } else {
          const result_3 = [...result, ...[(55296 + Math.floor(((value - 65536) / 1024))), (56320 + ((value - 65536) % 1024))]];
          const at_3 = (at + 1);
          [values, result, at] = [values, result_3, at_3];
          continue;
        }
      }
    } else {
      return ({ $: "ml_temporary_codePointUnitsUtf16Text_loop19_done", ["result"]: result });
    }
  }
}

function source_ml_temporary_displayPointsUtf16Text_loop20(text, result, at) {
  for (;;) {
    if ((at < text.length)) {
      const point = ((text)[at] ?? 65533);
      const source_ml_source1_s15 = (((point >= 55296) && (point <= 56319)) ? (() => {
        const low = ((text)[(at + 1)] ?? (-1));
        if (((low >= 56320) && (low <= 57343))) {
          const point_2 = (((65536 + ((point - 55296) * 1024)) + low) - 56320);
          const at_2 = (at + 1);
          return ({ $: "ml_temporary_displayPointsUtf16Text_join21_next", ["at"]: at_2, ["point"]: point_2 });
        } else {
          const point_3 = 65533;
          return ({ $: "ml_temporary_displayPointsUtf16Text_join21_next", ["at"]: at, ["point"]: point_3 });
        }
      })() : (((point >= 56320) && (point <= 57343)) ? (() => {
        const point_4 = 65533;
        return ({ $: "ml_temporary_displayPointsUtf16Text_join21_next", ["at"]: at, ["point"]: point_4 });
      })() : ({ $: "ml_temporary_displayPointsUtf16Text_join21_next", ["at"]: at, ["point"]: point })));
      switch (source_ml_source1_s15.$) {
        case "ml_temporary_displayPointsUtf16Text_join21_next": {
          const at_3 = source_ml_source1_s15["at"];
          const point_5 = source_ml_source1_s15["point"];
          const result_2 = [...result, ...[point_5]];
          const at_4 = (at_3 + 1);
          [text, result, at] = [text, result_2, at_4];
          continue;
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s15.$}`);
        }
      }
    } else {
      return result;
    }
  }
}

function source_ml_temporary_displayUtf8FromUtf16Text_loop22(bytes, points, at) {
  for (;;) {
    if ((at < points.length)) {
      const point = ((points)[at] ?? 65533);
      if ((point < 128)) {
        const bytes_2 = [...bytes, ...[point]];
        const at_2 = (at + 1);
        [bytes, points, at] = [bytes_2, points, at_2];
        continue;
      } else {
        if ((point < 2048)) {
          const bytes_3 = [...bytes, ...[(192 + Math.floor((point / 64))), (128 + (point % 64))]];
          const at_3 = (at + 1);
          [bytes, points, at] = [bytes_3, points, at_3];
          continue;
        } else {
          if ((point < 65536)) {
            const bytes_4 = [...bytes, ...[(224 + Math.floor((point / 4096))), (128 + (Math.floor((point / 64)) % 64)), (128 + (point % 64))]];
            const at_4 = (at + 1);
            [bytes, points, at] = [bytes_4, points, at_4];
            continue;
          } else {
            const bytes_5 = [...bytes, ...[(240 + Math.floor((point / 262144))), (128 + (Math.floor((point / 4096)) % 64)), (128 + (Math.floor((point / 64)) % 64)), (128 + (point % 64))]];
            const at_5 = (at + 1);
            [bytes, points, at] = [bytes_5, points, at_5];
            continue;
          }
        }
      }
    } else {
      return bytes;
    }
  }
}

function source_ml_temporary_planUtf16TextSubstitution_loop23(replacement, result, start, at) {
  for (;;) {
    if ((at < replacement.length)) {
      const next = ((replacement)[(at + 1)] ?? (-1));
      if (((((replacement)[at] ?? 0) === 36) && ((((next === 36) || (next === 38)) || (next === 96)) || (next === 39)))) {
        const result_3 = ((start < at) ? (() => {
          const result_2 = [...result, ...[0, start, at]];
          return result_2;
        })() : result);
        const result_6 = ((next === 36) ? (() => {
          const result_4 = [...result_3, ...[0, at, (at + 1)]];
          return result_4;
        })() : (() => {
          const result_5 = [...result_3, ...[((next === 38) ? 1 : ((next === 96) ? 2 : 3)), 0, 0]];
          return result_5;
        })());
        const at_2 = (at + 2);
        const start_2 = at_2;
        [replacement, result, start, at] = [replacement, result_6, start_2, at_2];
        continue;
      } else {
        const at_3 = (at + 1);
        [replacement, result, start, at] = [replacement, result, start, at_3];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_planUtf16TextSubstitution_loop23_done", ["result"]: result, ["start"]: start });
    }
  }
}

function source_ml_temporary_replaceUtf16Text_loop25(text, result, at) {
  for (;;) {
    if ((at < text.length)) {
      const result_2 = [...result, ...[((text)[at] ?? 0)]];
      const at_2 = (at + 1);
      [text, result, at] = [text, result_2, at_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_replaceUtf16Text_loop26(text, result, found, at) {
  for (;;) {
    if ((at < found)) {
      const result_2 = [...result, ...[((text)[at] ?? 0)]];
      const at_2 = (at + 1);
      [text, result, found, at] = [text, result_2, found, at_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_replaceUtf16Text_loop28(result, last, source, at) {
  for (;;) {
    if ((at < last)) {
      const result_2 = [...result, ...[((source)[at] ?? 0)]];
      const at_2 = (at + 1);
      [result, last, source, at] = [result_2, last, source, at_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_replaceUtf16Text_loop27(text, search, replacement, result, plan, found, instruction) {
  for (;;) {
    if ((instruction < plan.length)) {
      const kind = ((plan)[instruction] ?? 0);
      const first = ((kind === 0) ? ((plan)[(instruction + 1)] ?? 0) : ((kind === 2) ? 0 : ((kind === 1) ? found : (found + search.length))));
      const last = ((kind === 0) ? ((plan)[(instruction + 2)] ?? 0) : ((kind === 2) ? found : ((kind === 1) ? (found + search.length) : text.length)));
      const source = ((kind === 0) ? replacement : text);
      const at = first;
      const result_2 = source_ml_temporary_replaceUtf16Text_loop28(result, last, source, at);
      const instruction_2 = (instruction + 3);
      [text, search, replacement, result, plan, found, instruction] = [text, search, replacement, result_2, plan, found, instruction_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_replaceUtf16Text_loop24(text, search, replacement, all, result, plan, start, from, searching) {
  for (;;) {
    if (searching) {
      const found = indexUtf16Text(text, search, from);
      if ((found < 0)) {
        const searching_2 = false;
        [text, search, replacement, all, result, plan, start, from, searching] = [text, search, replacement, all, result, plan, start, from, searching_2];
        continue;
      } else {
        const at = start;
        const result_2 = source_ml_temporary_replaceUtf16Text_loop26(text, result, found, at);
        const instruction = 0;
        const result_3 = source_ml_temporary_replaceUtf16Text_loop27(text, search, replacement, result_2, plan, found, instruction);
        const start_2 = (found + search.length);
        const searching_3 = (all && !((search.length === 0) && (found === text.length)));
        const from_2 = (start_2 + ((search.length === 0) ? 1 : 0));
        [text, search, replacement, all, result, plan, start, from, searching] = [text, search, replacement, all, result_3, plan, start_2, from_2, searching_3];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_replaceUtf16Text_loop24_done", ["result"]: result, ["start"]: start });
    }
  }
}

function main() {

}

main();

return { replace: replaceUtf16Text, substitutionPlan: planUtf16TextSubstitution, split: splitUtf16Text, slicePosition: readUtf16TextSlicePosition, slice: sliceUtf16Text, substring: substringUtf16Text, character: characterUtf16Text, relativeCharacter: relativeCharacterUtf16Text, codePoint: codePointUtf16Text, points: pointsUtf16Text, index: indexUtf16Text, lastIndex: lastIndexUtf16Text, compare: compareUtf16Text, starts: startsUtf16Text, ends: endsUtf16Text, includes: includesUtf16Text, trim: trimUtf16Text, characterUnits: characterUnitsUtf16Text, codePointUnits: codePointUnitsUtf16Text, join: joinUtf16Text, displayUtf8: displayUtf8FromUtf16Text, displayPoints: displayPointsUtf16Text, fromUtf8: readUtf8TextUnits };
})();
