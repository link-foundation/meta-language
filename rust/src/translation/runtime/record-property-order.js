const ml_record_property_order = (() => {
// Translated from JavaScript by meta-language: portable core, JavaScript target.
'use strict';

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
  const source_ml_source1_s10 = source_ml_temporary_replaceUtf16Text_loop24(text, search, replacement, all, result, plan, start, from);
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

function source_ml_temporary_replaceUtf16Text_loop28(text, replacement, result, kind, last, at) {
  for (;;) {
    if ((at < last)) {
      const result_2 = [...result, ...[((kind === 0) ? ((replacement)[at] ?? 0) : ((text)[at] ?? 0))]];
      const at_2 = (at + 1);
      [text, replacement, result, kind, last, at] = [text, replacement, result_2, kind, last, at_2];
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
      const at = first;
      const result_2 = source_ml_temporary_replaceUtf16Text_loop28(text, replacement, result, kind, last, at);
      const instruction_2 = (instruction + 3);
      [text, search, replacement, result, plan, found, instruction] = [text, search, replacement, result_2, plan, found, instruction_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_replaceUtf16Text_loop24(text, search, replacement, all, result, plan, start, from) {
  for (;;) {
    const found = indexUtf16Text(text, search, from);
    if ((found < 0)) {
      return ({ $: "ml_temporary_replaceUtf16Text_loop24_done", ["result"]: result, ["start"]: start });
    } else {
      const at = start;
      const result_2 = source_ml_temporary_replaceUtf16Text_loop26(text, result, found, at);
      const instruction = 0;
      const result_3 = source_ml_temporary_replaceUtf16Text_loop27(text, search, replacement, result_2, plan, found, instruction);
      const start_2 = (found + search.length);
      if ((!all || ((search.length === 0) && (found === text.length)))) {
        return ({ $: "ml_temporary_replaceUtf16Text_loop24_done", ["result"]: result_3, ["start"]: start_2 });
      } else {
        const from_2 = (start_2 + ((search.length === 0) ? 1 : 0));
        [text, search, replacement, all, result, plan, start, from] = [text, search, replacement, all, result_3, plan, start_2, from_2];
        continue;
      }
    }
  }
}

function main() {

}

main();

return { replace: replaceUtf16Text, substitutionPlan: planUtf16TextSubstitution, split: splitUtf16Text, slicePosition: readUtf16TextSlicePosition, slice: sliceUtf16Text, substring: substringUtf16Text, character: characterUtf16Text, relativeCharacter: relativeCharacterUtf16Text, codePoint: codePointUtf16Text, points: pointsUtf16Text, index: indexUtf16Text, lastIndex: lastIndexUtf16Text, compare: compareUtf16Text, starts: startsUtf16Text, ends: endsUtf16Text, includes: includesUtf16Text, trim: trimUtf16Text, characterUnits: characterUnitsUtf16Text, codePointUnits: codePointUnitsUtf16Text, join: joinUtf16Text, displayUtf8: displayUtf8FromUtf16Text, displayPoints: displayPointsUtf16Text, fromUtf8: readUtf8TextUnits };
})();


function refuseRequiredNumberFact() {
  return ((ml_message,ml_present)=>{throw new RangeError(ml_present?ml_message:undefined);})("missing required Number fact column value",true);
}

function refuseRequiredTextFact() {
  return ((ml_message,ml_present)=>{throw new RangeError(ml_present?ml_message:undefined);})("missing required text fact column value",true);
}

function refuseRequiredBooleanFact() {
  return ((ml_message,ml_present)=>{throw new RangeError(ml_present?ml_message:undefined);})("missing required Boolean fact column value",true);
}

function readRequiredNumberFact(values, index) {
  if ((((index < 0) || ((index % 1) !== 0)) || (index >= values.length))) {
    return ((ml_message,ml_present)=>{throw new RangeError(ml_present?ml_message:undefined);})("invalid required Number fact column position",true);
  } else {
    return ((values)[index] ?? refuseRequiredNumberFact());
  }
}

function readRequiredTextFact(values, index) {
  if ((((index < 0) || ((index % 1) !== 0)) || (index >= values.length))) {
    return ((ml_message,ml_present)=>{throw new RangeError(ml_present?ml_message:undefined);})("invalid required text fact column position",true);
  } else {
    return ((values)[index] ?? refuseRequiredTextFact());
  }
}

function readRequiredBooleanFact(values, index) {
  if ((((index < 0) || ((index % 1) !== 0)) || (index >= values.length))) {
    return ((ml_message,ml_present)=>{throw new RangeError(ml_present?ml_message:undefined);})("invalid required Boolean fact column position",true);
  } else {
    return ((values)[index] ?? refuseRequiredBooleanFact());
  }
}

function refuseRequiredRecordName() {
  return ((ml_message,ml_present)=>{throw new RangeError(ml_present?ml_message:undefined);})("missing required record name row",true);
}

function readRequiredRecordName(names, index) {
  if ((((index < 0) || ((index % 1) !== 0)) || (index >= names.length))) {
    return ((ml_message,ml_present)=>{throw new RangeError(ml_present?ml_message:undefined);})("invalid required record name position",true);
  } else {
    return ((names)[index] ?? refuseRequiredRecordName());
  }
}

function readRecordValueOrder(count, reverse) {
  if ((!Number.isSafeInteger(count) || (count < 0))) {
    return [(-1)];
  } else {
    const order = [];
    const index = 0;
    const order_2 = source_ml_temporary_readRecordValueOrder_loop1(count, reverse, order, index);
    return order_2;
  }
}

function readRecordProjection() {
  return "checked ordered record values";
}

function readRecordProjectionError(projection, names, entries) {
  if ((projection !== readRecordProjection())) {
    return "unknown checked record projection";
  } else {
    if ((names.length !== entries.length)) {
      return "checked record projection has inconsistent entry facts";
    } else {
      const index = 0;
      const source_ml_source1_s1 = source_ml_temporary_readRecordProjectionError_loop2(names, entries, index);
      switch (source_ml_source1_s1.$) {
        case "ml_temporary_readRecordProjectionError_loop2_done": {
          return "";
        }
        case "ml_temporary_readRecordProjectionError_loop2_return": {
          const source_ml_temporary_result2 = source_ml_source1_s1["ml_temporary_result2"];
          return source_ml_temporary_result2;
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s1.$}`);
        }
      }
    }
  }
}

function acceptRecordFieldName(name) {
  const index = 0;
  const source_ml_source1_s2 = source_ml_temporary_acceptRecordFieldName_loop3(name, index);
  switch (source_ml_source1_s2.$) {
    case "ml_temporary_acceptRecordFieldName_loop3_done": {
      return true;
    }
    case "ml_temporary_acceptRecordFieldName_loop3_return": {
      const source_ml_temporary_result3 = source_ml_source1_s2["ml_temporary_result3"];
      return source_ml_temporary_result3;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s2.$}`);
    }
  }
}

function compareRecordFieldNames(left, right) {
  const end = ((left.length < right.length) ? left.length : right.length);
  const index = 0;
  const source_ml_source1_s3 = source_ml_temporary_compareRecordFieldNames_loop4(left, right, end, index);
  switch (source_ml_source1_s3.$) {
    case "ml_temporary_compareRecordFieldNames_loop4_done": {
      if ((left.length < right.length)) {
        return (-1);
      } else {
        if ((left.length > right.length)) {
          return 1;
        } else {
          return 0;
        }
      }
    }
    case "ml_temporary_compareRecordFieldNames_loop4_return": {
      const source_ml_temporary_result4 = source_ml_source1_s3["ml_temporary_result4"];
      return source_ml_temporary_result4;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s3.$}`);
    }
  }
}

function readRecordFieldPosition(fields, name) {
  if (!acceptRecordFieldName(name)) {
    return (-1);
  } else {
    const index = 0;
    const source_ml_source1_s4 = source_ml_temporary_readRecordFieldPosition_loop5(fields, index);
    switch (source_ml_source1_s4.$) {
      case "ml_temporary_readRecordFieldPosition_loop5_done": {
        const index_2 = 0;
        const source_ml_source1_s5 = source_ml_temporary_readRecordFieldPosition_loop6(fields, name, index_2);
        switch (source_ml_source1_s5.$) {
          case "ml_temporary_readRecordFieldPosition_loop6_done": {
            return (-1);
          }
          case "ml_temporary_readRecordFieldPosition_loop6_return": {
            const source_ml_temporary_result6 = source_ml_source1_s5["ml_temporary_result6"];
            return source_ml_temporary_result6;
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source1_s5.$}`);
          }
        }
      }
      case "ml_temporary_readRecordFieldPosition_loop5_return": {
        const source_ml_temporary_result6_2 = source_ml_source1_s4["ml_temporary_result6"];
        return source_ml_temporary_result6_2;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s4.$}`);
      }
    }
  }
}

function readRecordSortedFieldPositions(fields) {
  const index = 0;
  const source_ml_source1_s6 = source_ml_temporary_readRecordSortedFieldPositions_loop7(fields, index);
  switch (source_ml_source1_s6.$) {
    case "ml_temporary_readRecordSortedFieldPositions_loop7_done": {
      const sorted = readRecordValueOrder(fields.length, false);
      const width = 1;
      const sorted_2 = source_ml_temporary_readRecordSortedFieldPositions_loop8(fields, sorted, width);
      return sorted_2;
    }
    case "ml_temporary_readRecordSortedFieldPositions_loop7_return": {
      const source_ml_temporary_result10 = source_ml_source1_s6["ml_temporary_result10"];
      return source_ml_temporary_result10;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s6.$}`);
    }
  }
}

function readRecordFieldOrder(fields) {
  const sorted = readRecordSortedFieldPositions(fields);
  if (((sorted.length > 0) && (readRequiredNumberFact(sorted, 0) < 0))) {
    return [(-1)];
  } else {
    const selected = [];
    const index = 0;
    const selected_2 = source_ml_temporary_readRecordFieldOrder_loop11(fields, sorted, selected, index);
    return selected_2;
  }
}

function readRecordFieldInsertionOrder(fields) {
  const sorted = readRecordSortedFieldPositions(fields);
  if (((sorted.length > 0) && (readRequiredNumberFact(sorted, 0) < 0))) {
    return [(-1)];
  } else {
    const first = [];
    const last = [];
    const start = 0;
    const index = 0;
    const source_ml_source1_s7 = source_ml_temporary_readRecordFieldInsertionOrder_loop12(fields, sorted, first, last, start, index);
    switch (source_ml_source1_s7.$) {
      case "ml_temporary_readRecordFieldInsertionOrder_loop12_done": {
        const first_2 = source_ml_source1_s7["first"];
        const last_2 = source_ml_source1_s7["last"];
        const selected = [];
        const index_2 = 0;
        const selected_2 = source_ml_temporary_readRecordFieldInsertionOrder_loop13(fields, first_2, last_2, selected, index_2);
        return selected_2;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s7.$}`);
      }
    }
  }
}

function readRecordSpreadRole(kind) {
  if ((kind === "var")) {
    return "open";
  } else {
    if ((kind === "record")) {
      return "fields";
    } else {
      return "refuse";
    }
  }
}

function readRecordTypeConstraintStep(left, right, leftOpen, rightOpen, phase, index) {
  if ((phase < 1)) {
    if ((index >= left.length)) {
      return [5, (-1), (-1), 1, 0];
    } else {
      const matching = readRecordFieldPosition(right, readRequiredRecordName(left, index));
      if ((matching >= 0)) {
        return [0, index, matching, 0, (index + 1)];
      } else {
        return [(rightOpen ? 1 : 3), index, (-1), 0, (index + 1)];
      }
    }
  } else {
    if ((index >= right.length)) {
      return [6, (-1), (-1), 1, index];
    } else {
      const matching_2 = readRecordFieldPosition(left, readRequiredRecordName(right, index));
      if ((matching_2 >= 0)) {
        return [5, matching_2, index, 1, (index + 1)];
      } else {
        return [(leftOpen ? 2 : 4), (-1), index, 1, (index + 1)];
      }
    }
  }
}

function readRecordTypeConstraintError(action, name) {
  if (((action === 3) || (action === 4))) {
    return (("record field " + name) + " is missing");
  } else {
    return "";
  }
}

function readRecordMethodError(kind, operation, parameters, argumentsCount) {
  if ((kind !== "function")) {
    return ("record has no method " + operation);
  } else {
    if ((argumentsCount !== parameters)) {
      return (((("." + operation) + "() expects ") + String(parameters)) + " arguments");
    } else {
      return "";
    }
  }
}

function readRecordFieldInferenceStep(fields, name, open) {
  const index = readRecordFieldPosition(fields, name);
  if ((index >= 0)) {
    return [0, index];
  } else {
    return [(open ? 1 : 2), (-1)];
  }
}

function recordTypeFactField(starts, counts, labels, targets, owner, name) {
  if ((owner < 0)) {
    return (-1);
  } else {
    const start = readRequiredNumberFact(starts, owner);
    const end = (start + readRequiredNumberFact(counts, owner));
    const edge = start;
    const source_ml_source1_s8 = source_ml_temporary_recordTypeFactField_loop15(labels, targets, name, end, edge);
    switch (source_ml_source1_s8.$) {
      case "ml_temporary_recordTypeFactField_loop15_done": {
        return (-1);
      }
      case "ml_temporary_recordTypeFactField_loop15_return": {
        const source_ml_temporary_result15 = source_ml_source1_s8["ml_temporary_result15"];
        return source_ml_temporary_result15;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s8.$}`);
      }
    }
  }
}

function recordTypeFactKind(texts, starts, counts, labels, targets, owner) {
  const kind = recordTypeFactField(starts, counts, labels, targets, owner, "kind");
  if ((kind < 0)) {
    return "";
  } else {
    return readRequiredTextFact(texts, kind);
  }
}

function recordTypeFactElements(starts, counts, targets, owner) {
  const values = [];
  if ((owner < 0)) {
    return values;
  } else {
    const start = readRequiredNumberFact(starts, owner);
    const end = (start + readRequiredNumberFact(counts, owner));
    const edge = start;
    const values_2 = source_ml_temporary_recordTypeFactElements_loop16(targets, values, end, edge);
    return values_2;
  }
}

function readRecordTypeOwnerPresence(texts, numbers, starts, counts, labels, targets, bindings, root, owner) {
  const pending = [root];
  const seen = [];
  const position = 0;
  const source_ml_source1_s9 = source_ml_temporary_readRecordTypeOwnerPresence_loop17(texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position);
  switch (source_ml_source1_s9.$) {
    case "ml_temporary_readRecordTypeOwnerPresence_loop17_done": {
      return false;
    }
    case "ml_temporary_readRecordTypeOwnerPresence_loop17_return": {
      const source_ml_temporary_result17 = source_ml_source1_s9["ml_temporary_result17"];
      return source_ml_temporary_result17;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s9.$}`);
    }
  }
}

function readJavaScriptLiteralPropertyIndex(key) {
  if (((key.length === 0) || (key.length > 10))) {
    return (-1);
  } else {
    if (((key.length > 1) && (() => {
      const source_ml_source1_comparison_operand10 = (key)[0];
      const source_ml_source1_comparison_operand11 = 48;
      return (source_ml_source1_comparison_operand10 === source_ml_source1_comparison_operand11);
    })())) {
      return (-1);
    } else {
      const value = 0;
      const at = 0;
      const source_ml_source1_s12 = source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22(key, value, at);
      switch (source_ml_source1_s12.$) {
        case "ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done": {
          const value_2 = source_ml_source1_s12["value"];
          if ((value_2 <= 4294967294)) {
            return value_2;
          } else {
            return (-1);
          }
        }
        case "ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_return": {
          const source_ml_temporary_result22 = source_ml_source1_s12["ml_temporary_result22"];
          return source_ml_temporary_result22;
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s12.$}`);
        }
      }
    }
  }
}

function readJavaScriptLiteralPropertyForm(kind, index) {
  if (((((kind === "array") || (kind === "tuple")) || (kind === "string")) && (index >= 0))) {
    return "index";
  } else {
    return "field";
  }
}

function readJavaScriptStringIndexNumberForm(kind, element) {
  if ((kind === "float")) {
    return "identity";
  } else {
    if (((kind === "int") || (kind === "nat"))) {
      return "convert";
    } else {
      if (((kind === "unit") || ((kind === "option") && (element === "unit")))) {
        return "absent";
      } else {
        if (((kind === "option") && (((element === "float") || (element === "int")) || (element === "nat")))) {
          return "optional";
        } else {
          return "refuse";
        }
      }
    }
  }
}

function refuseJavaScriptRecordOrderName() {
  return ((ml_message,ml_present)=>{throw new TypeError(ml_present?ml_message:undefined);})("an ordered record needs every actual own field name",true);
}

function refuseJavaScriptRecordOrderPosition() {
  return ((ml_message,ml_present)=>{throw new TypeError(ml_present?ml_message:undefined);})("an ordered record needs every actual field position",true);
}

function readJavaScriptRecordOrderName(names, index) {
  return ((names)[index] ?? refuseJavaScriptRecordOrderName());
}

function readJavaScriptRecordOrderPosition(positions, index) {
  return ((positions)[index] ?? refuseJavaScriptRecordOrderPosition());
}

function acceptJavaScriptRecordOrderNames(names) {
  const index = 0;
  const source_ml_source1_s13 = source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23(names, index);
  switch (source_ml_source1_s13.$) {
    case "ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done": {
      return true;
    }
    case "ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return": {
      const source_ml_temporary_result23 = source_ml_source1_s13["ml_temporary_result23"];
      return source_ml_temporary_result23;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s13.$}`);
    }
  }
}

function acceptJavaScriptRecordOrderPermutation(count, order) {
  if ((((!Number.isSafeInteger(count) || (count < 0)) || ((count === 0) && ((1 / count) < 0))) || (order.length !== count))) {
    return false;
  } else {
    const index = 0;
    const source_ml_source1_s14 = source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24(count, order, index);
    switch (source_ml_source1_s14.$) {
      case "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done": {
        return true;
      }
      case "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return": {
        const source_ml_temporary_result24 = source_ml_source1_s14["ml_temporary_result24"];
        return source_ml_temporary_result24;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s14.$}`);
      }
    }
  }
}

function readJavaScriptRecordOrderCreationError(names, insertion) {
  if ((!acceptJavaScriptRecordOrderNames(names) || !acceptJavaScriptRecordOrderPermutation(names.length, insertion))) {
    return "invalid ordered record field sequence";
  } else {
    return "";
  }
}

function createJavaScriptRecordPropertyOrder(names, insertion) {
  const error = readJavaScriptRecordOrderCreationError(names, insertion);
  if (!!(error)) {
    return ((ml_message,ml_present)=>{throw new TypeError(ml_present?ml_message:undefined);})(error,true);
  } else {
    const numeric = [];
    const field = 0;
    const numeric_2 = source_ml_temporary_createJavaScriptRecordPropertyOrder_loop26(names, numeric, field);
    const result = numeric_2;
    const index = 0;
    const result_2 = source_ml_temporary_createJavaScriptRecordPropertyOrder_loop28(names, insertion, result, index);
    return result_2;
  }
}

function acceptJavaScriptRecordPropertyOrder(names, order) {
  if ((!acceptJavaScriptRecordOrderNames(names) || !acceptJavaScriptRecordOrderPermutation(names.length, order))) {
    return false;
  } else {
    const expected = createJavaScriptRecordPropertyOrder(names, order);
    const index = 0;
    const source_ml_source1_s15 = source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29(order, expected, index);
    switch (source_ml_source1_s15.$) {
      case "ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done": {
        return true;
      }
      case "ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_return": {
        const source_ml_temporary_result29 = source_ml_source1_s15["ml_temporary_result29"];
        return source_ml_temporary_result29;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s15.$}`);
      }
    }
  }
}

function readJavaScriptRecordOrderMergeError(names, chunks) {
  if (!acceptJavaScriptRecordOrderNames(names)) {
    return "invalid ordered record field names";
  } else {
    const insertion = [];
    const chunk = 0;
    const source_ml_source1_s16 = source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30(names, chunks, insertion, chunk);
    switch (source_ml_source1_s16.$) {
      case "ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done": {
        const insertion_2 = source_ml_source1_s16["insertion"];
        return readJavaScriptRecordOrderCreationError(names, insertion_2);
      }
      case "ml_temporary_readJavaScriptRecordOrderMergeError_loop30_return": {
        const source_ml_temporary_result30 = source_ml_source1_s16["ml_temporary_result30"];
        return source_ml_temporary_result30;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s16.$}`);
      }
    }
  }
}

function mergeJavaScriptRecordPropertyOrder(names, chunks) {
  const error = readJavaScriptRecordOrderMergeError(names, chunks);
  if (!!(error)) {
    return ((ml_message,ml_present)=>{throw new TypeError(ml_present?ml_message:undefined);})(error,true);
  } else {
    const insertion = [];
    const chunk = 0;
    const insertion_2 = source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop32(chunks, insertion, chunk);
    return createJavaScriptRecordPropertyOrder(names, insertion_2);
  }
}

function readJavaScriptRecordOrderRebaseError(target, source, order) {
  if ((!acceptJavaScriptRecordOrderNames(target) || !acceptJavaScriptRecordPropertyOrder(source, order))) {
    return "invalid ordered record spread source";
  } else {
    const index = 0;
    const source_ml_source1_s17 = source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34(target, source, index);
    switch (source_ml_source1_s17.$) {
      case "ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done": {
        return "";
      }
      case "ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_return": {
        const source_ml_temporary_result34 = source_ml_source1_s17["ml_temporary_result34"];
        return source_ml_temporary_result34;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s17.$}`);
      }
    }
  }
}

function rebaseJavaScriptRecordPropertyOrder(target, source, order) {
  const error = readJavaScriptRecordOrderRebaseError(target, source, order);
  if (!!(error)) {
    return ((ml_message,ml_present)=>{throw new TypeError(ml_present?ml_message:undefined);})(error,true);
  } else {
    const result = [];
    const index = 0;
    const result_2 = source_ml_temporary_rebaseJavaScriptRecordPropertyOrder_loop35(target, source, order, result, index);
    return result_2;
  }
}

function readJavaScriptRecordOrderKeyError(names, order) {
  if (acceptJavaScriptRecordPropertyOrder(names, order)) {
    return "";
  } else {
    return "invalid ordered record property order";
  }
}

function readJavaScriptRecordPropertyKeys(names, order) {
  const error = readJavaScriptRecordOrderKeyError(names, order);
  if (!!(error)) {
    return ((ml_message,ml_present)=>{throw new TypeError(ml_present?ml_message:undefined);})(error,true);
  } else {
    const result = [];
    const index = 0;
    const result_2 = source_ml_temporary_readJavaScriptRecordPropertyKeys_loop36(names, order, result, index);
    return result_2;
  }
}

function readJavaScriptRecordPropertyKeyTexts(names, order) {
  const rows = readJavaScriptRecordPropertyKeys(names, order);
  const result = [];
  const index = 0;
  const result_2 = source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop37(rows, result, index);
  return result_2;
}

function readJavaScriptRecordPropertyIndex(names, key) {
  const units = [];
  const index = 0;
  const units_2 = source_ml_temporary_readJavaScriptRecordPropertyIndex_loop39(key, units, index);
  return readRecordFieldPosition(names, units_2);
}

function createJavaScriptDensePropertyKeyTexts(length) {
  if (((!Number.isSafeInteger(length) || (length < 0)) || ((length === 0) && ((1 / length) < 0)))) {
    return ((ml_message,ml_present)=>{throw new TypeError(ml_present?ml_message:undefined);})("invalid dense own-field length",true);
  } else {
    const result = [];
    const index = 0;
    const result_2 = source_ml_temporary_createJavaScriptDensePropertyKeyTexts_loop40(length, result, index);
    return result_2;
  }
}

function createJavaScriptStringOwnPropertyValues(text) {
  const result = [];
  const index = 0;
  const result_2 = source_ml_temporary_createJavaScriptStringOwnPropertyValues_loop41(text, result, index);
  return result_2;
}

function source_ml_temporary_readRecordValueOrder_loop1(count, reverse, order, index) {
  for (;;) {
    if ((index < count)) {
      const order_2 = [...order, ...[(reverse ? ((count - index) - 1) : index)]];
      const index_2 = (index + 1);
      [count, reverse, order, index] = [count, reverse, order_2, index_2];
      continue;
    } else {
      return order;
    }
  }
}

function source_ml_temporary_readRecordProjectionError_loop2(names, entries, index) {
  for (;;) {
    if ((index < names.length)) {
      if (!!!((entries)[index])) {
        return ({ $: "ml_temporary_readRecordProjectionError_loop2_return", ["ml_temporary_result2"]: "checked record projection has a malformed field entry" });
      } else {
        if (!acceptRecordFieldName(readRequiredRecordName(names, index))) {
          return ({ $: "ml_temporary_readRecordProjectionError_loop2_return", ["ml_temporary_result2"]: "checked record projection has an invalid UTF16 field name" });
        } else {
          if (((index > 0) && (compareRecordFieldNames(readRequiredRecordName(names, (index - 1)), readRequiredRecordName(names, index)) >= 0))) {
            return ({ $: "ml_temporary_readRecordProjectionError_loop2_return", ["ml_temporary_result2"]: "checked record projection needs unique fields in UTF16 order" });
          } else {
            const index_2 = (index + 1);
            [names, entries, index] = [names, entries, index_2];
            continue;
          }
        }
      }
    } else {
      return ({ $: "ml_temporary_readRecordProjectionError_loop2_done" });
    }
  }
}

function source_ml_temporary_acceptRecordFieldName_loop3(name, index) {
  for (;;) {
    if ((index < name.length)) {
      const unit = (name)[index];
      if (((!Number.isInteger(unit) || (() => {
        const source_ml_source1_comparison_operand18 = unit;
        const source_ml_source1_comparison_operand19 = 0;
        if ((source_ml_source1_comparison_operand18 === undefined)) {
          return (NaN < source_ml_source1_comparison_operand19);
        } else {
          return (source_ml_source1_comparison_operand18 < source_ml_source1_comparison_operand19);
        }
      })()) || (() => {
        const source_ml_source1_comparison_operand20 = unit;
        const source_ml_source1_comparison_operand21 = 65535;
        if ((source_ml_source1_comparison_operand20 === undefined)) {
          return (NaN > source_ml_source1_comparison_operand21);
        } else {
          return (source_ml_source1_comparison_operand20 > source_ml_source1_comparison_operand21);
        }
      })())) {
        return ({ $: "ml_temporary_acceptRecordFieldName_loop3_return", ["ml_temporary_result3"]: false });
      } else {
        const index_2 = (index + 1);
        [name, index] = [name, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_acceptRecordFieldName_loop3_done" });
    }
  }
}

function source_ml_temporary_compareRecordFieldNames_loop4(left, right, end, index) {
  for (;;) {
    if ((index < end)) {
      if ((readRequiredNumberFact(left, index) < readRequiredNumberFact(right, index))) {
        return ({ $: "ml_temporary_compareRecordFieldNames_loop4_return", ["ml_temporary_result4"]: (-1) });
      } else {
        if ((readRequiredNumberFact(left, index) > readRequiredNumberFact(right, index))) {
          return ({ $: "ml_temporary_compareRecordFieldNames_loop4_return", ["ml_temporary_result4"]: 1 });
        } else {
          const index_2 = (index + 1);
          [left, right, end, index] = [left, right, end, index_2];
          continue;
        }
      }
    } else {
      return ({ $: "ml_temporary_compareRecordFieldNames_loop4_done" });
    }
  }
}

function source_ml_temporary_readRecordFieldPosition_loop6(fields, name, index) {
  for (;;) {
    if ((index < fields.length)) {
      const comparison = compareRecordFieldNames(readRequiredRecordName(fields, index), name);
      if ((!(comparison < 0) && !(comparison > 0))) {
        return ({ $: "ml_temporary_readRecordFieldPosition_loop6_return", ["ml_temporary_result6"]: index });
      } else {
        const index_2 = (index + 1);
        [fields, name, index] = [fields, name, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_readRecordFieldPosition_loop6_done" });
    }
  }
}

function source_ml_temporary_readRecordFieldPosition_loop5(fields, index) {
  for (;;) {
    if ((index < fields.length)) {
      if (!acceptRecordFieldName(readRequiredRecordName(fields, index))) {
        return ({ $: "ml_temporary_readRecordFieldPosition_loop5_return", ["ml_temporary_result6"]: (-1) });
      } else {
        const index_2 = (index + 1);
        [fields, index] = [fields, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_readRecordFieldPosition_loop5_done" });
    }
  }
}

function source_ml_temporary_readRecordSortedFieldPositions_loop10(fields, sorted, merged, middle, end, left, right) {
  for (;;) {
    if (((left < middle) || (right < end))) {
      if (((right >= end) || ((left < middle) && (compareRecordFieldNames(readRequiredRecordName(fields, readRequiredNumberFact(sorted, left)), readRequiredRecordName(fields, readRequiredNumberFact(sorted, right))) <= 0)))) {
        const merged_2 = [...merged, ...[readRequiredNumberFact(sorted, left)]];
        const left_2 = (left + 1);
        [fields, sorted, merged, middle, end, left, right] = [fields, sorted, merged_2, middle, end, left_2, right];
        continue;
      } else {
        const merged_3 = [...merged, ...[readRequiredNumberFact(sorted, right)]];
        const right_2 = (right + 1);
        [fields, sorted, merged, middle, end, left, right] = [fields, sorted, merged_3, middle, end, left, right_2];
        continue;
      }
    } else {
      return merged;
    }
  }
}

function source_ml_temporary_readRecordSortedFieldPositions_loop9(fields, sorted, width, merged, start) {
  for (;;) {
    if ((start < sorted.length)) {
      const middle = (((start + width) < sorted.length) ? (start + width) : sorted.length);
      const end = (((middle + width) < sorted.length) ? (middle + width) : sorted.length);
      const left = start;
      const right = middle;
      const merged_2 = source_ml_temporary_readRecordSortedFieldPositions_loop10(fields, sorted, merged, middle, end, left, right);
      const start_2 = (start + (width * 2));
      [fields, sorted, width, merged, start] = [fields, sorted, width, merged_2, start_2];
      continue;
    } else {
      return merged;
    }
  }
}

function source_ml_temporary_readRecordSortedFieldPositions_loop8(fields, sorted, width) {
  for (;;) {
    if ((width < sorted.length)) {
      const merged = [];
      const start = 0;
      const merged_2 = source_ml_temporary_readRecordSortedFieldPositions_loop9(fields, sorted, width, merged, start);
      const sorted_2 = merged_2;
      const width_2 = (width * 2);
      [fields, sorted, width] = [fields, sorted_2, width_2];
      continue;
    } else {
      return sorted;
    }
  }
}

function source_ml_temporary_readRecordSortedFieldPositions_loop7(fields, index) {
  for (;;) {
    if ((index < fields.length)) {
      if (!acceptRecordFieldName(readRequiredRecordName(fields, index))) {
        return ({ $: "ml_temporary_readRecordSortedFieldPositions_loop7_return", ["ml_temporary_result10"]: [(-1)] });
      } else {
        const index_2 = (index + 1);
        [fields, index] = [fields, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_readRecordSortedFieldPositions_loop7_done" });
    }
  }
}

function source_ml_temporary_readRecordFieldOrder_loop11(fields, sorted, selected, index) {
  for (;;) {
    if ((index < sorted.length)) {
      if ((((index + 1) >= sorted.length) || (compareRecordFieldNames(readRequiredRecordName(fields, readRequiredNumberFact(sorted, index)), readRequiredRecordName(fields, readRequiredNumberFact(sorted, (index + 1)))) < 0))) {
        const selected_2 = [...selected, ...[readRequiredNumberFact(sorted, index)]];
        const index_2 = (index + 1);
        [fields, sorted, selected, index] = [fields, sorted, selected_2, index_2];
        continue;
      } else {
        const index_3 = (index + 1);
        [fields, sorted, selected, index] = [fields, sorted, selected, index_3];
        continue;
      }
    } else {
      return selected;
    }
  }
}

function source_ml_temporary_readRecordFieldInsertionOrder_loop14(fields, first, last, selected, index, lower, upper) {
  for (;;) {
    if ((lower < upper)) {
      const middle = Math.floor(((lower + upper) / 2));
      const comparison = compareRecordFieldNames(readRequiredRecordName(fields, index), readRequiredRecordName(fields, readRequiredNumberFact(last, middle)));
      if ((comparison < 0)) {
        const upper_2 = middle;
        [fields, first, last, selected, index, lower, upper] = [fields, first, last, selected, index, lower, upper_2];
        continue;
      } else {
        if ((comparison > 0)) {
          const lower_2 = (middle + 1);
          [fields, first, last, selected, index, lower, upper] = [fields, first, last, selected, index, lower_2, upper];
          continue;
        } else {
          const selected_3 = ((!(readRequiredNumberFact(first, middle) < index) && !(readRequiredNumberFact(first, middle) > index)) ? (() => {
            const selected_2 = [...selected, ...[readRequiredNumberFact(last, middle)]];
            return selected_2;
          })() : selected);
          const lower_3 = upper;
          [fields, first, last, selected, index, lower, upper] = [fields, first, last, selected_3, index, lower_3, upper];
          continue;
        }
      }
    } else {
      return selected;
    }
  }
}

function source_ml_temporary_readRecordFieldInsertionOrder_loop13(fields, first, last, selected, index) {
  for (;;) {
    if ((index < fields.length)) {
      const lower = 0;
      const upper = last.length;
      const selected_2 = source_ml_temporary_readRecordFieldInsertionOrder_loop14(fields, first, last, selected, index, lower, upper);
      const index_2 = (index + 1);
      [fields, first, last, selected, index] = [fields, first, last, selected_2, index_2];
      continue;
    } else {
      return selected;
    }
  }
}

function source_ml_temporary_readRecordFieldInsertionOrder_loop12(fields, sorted, first, last, start, index) {
  for (;;) {
    if ((index < sorted.length)) {
      if ((((index + 1) >= sorted.length) || (compareRecordFieldNames(readRequiredRecordName(fields, readRequiredNumberFact(sorted, index)), readRequiredRecordName(fields, readRequiredNumberFact(sorted, (index + 1)))) < 0))) {
        const first_2 = [...first, ...[readRequiredNumberFact(sorted, start)]];
        const last_2 = [...last, ...[readRequiredNumberFact(sorted, index)]];
        const start_2 = (index + 1);
        const index_2 = (index + 1);
        [fields, sorted, first, last, start, index] = [fields, sorted, first_2, last_2, start_2, index_2];
        continue;
      } else {
        const index_3 = (index + 1);
        [fields, sorted, first, last, start, index] = [fields, sorted, first, last, start, index_3];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_readRecordFieldInsertionOrder_loop12_done", ["first"]: first, ["last"]: last });
    }
  }
}

function source_ml_temporary_recordTypeFactField_loop15(labels, targets, name, end, edge) {
  for (;;) {
    if ((edge < end)) {
      if ((readRequiredTextFact(labels, edge) === name)) {
        return ({ $: "ml_temporary_recordTypeFactField_loop15_return", ["ml_temporary_result15"]: readRequiredNumberFact(targets, edge) });
      } else {
        const edge_2 = (edge + 1);
        [labels, targets, name, end, edge] = [labels, targets, name, end, edge_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_recordTypeFactField_loop15_done" });
    }
  }
}

function source_ml_temporary_recordTypeFactElements_loop16(targets, values, end, edge) {
  for (;;) {
    if ((edge < end)) {
      const values_2 = [...values, ...[readRequiredNumberFact(targets, edge)]];
      const edge_2 = (edge + 1);
      [targets, values, end, edge] = [targets, values_2, end, edge_2];
      continue;
    } else {
      return values;
    }
  }
}

function source_ml_temporary_readRecordTypeOwnerPresence_loop19(seen, node, visited, index) {
  for (;;) {
    if ((index < seen.length)) {
      if ((readRequiredNumberFact(seen, index) === node)) {
        const visited_2 = true;
        const index_2 = (index + 1);
        [seen, node, visited, index] = [seen, node, visited_2, index_2];
        continue;
      } else {
        const index_3 = (index + 1);
        [seen, node, visited, index] = [seen, node, visited, index_3];
        continue;
      }
    } else {
      return visited;
    }
  }
}

function source_ml_temporary_readRecordTypeOwnerPresence_loop20(pending, children, index) {
  for (;;) {
    if ((index < children.length)) {
      const pending_2 = [...pending, ...[readRequiredNumberFact(children, index)]];
      const index_2 = (index + 1);
      [pending, children, index] = [pending_2, children, index_2];
      continue;
    } else {
      return pending;
    }
  }
}

function source_ml_temporary_readRecordTypeOwnerPresence_loop21(starts, counts, labels, targets, pending, fields, index) {
  for (;;) {
    if ((index < fields.length)) {
      const pending_2 = [...pending, ...[recordTypeFactField(starts, counts, labels, targets, readRequiredNumberFact(fields, index), "type")]];
      const index_2 = (index + 1);
      [starts, counts, labels, targets, pending, fields, index] = [starts, counts, labels, targets, pending_2, fields, index_2];
      continue;
    } else {
      return pending;
    }
  }
}

function source_ml_temporary_readRecordTypeOwnerPresence_loop18(texts, numbers, starts, counts, labels, targets, bindings, node, kind) {
  for (;;) {
    if ((kind === "var")) {
      const id = recordTypeFactField(starts, counts, labels, targets, node, "id");
      const variable = readRequiredNumberFact(numbers, id);
      if (((!Number.isInteger(variable) || (variable < 0)) || (variable >= bindings.length))) {
        return ({ $: "ml_temporary_readRecordTypeOwnerPresence_loop18_done", ["node"]: node, ["kind"]: kind });
      } else {
        const bound = readRequiredNumberFact(bindings, variable);
        const boundKind = recordTypeFactKind(texts, starts, counts, labels, targets, bound);
        if ((boundKind === "")) {
          return ({ $: "ml_temporary_readRecordTypeOwnerPresence_loop18_done", ["node"]: node, ["kind"]: kind });
        } else {
          const node_2 = bound;
          const kind_2 = boundKind;
          [texts, numbers, starts, counts, labels, targets, bindings, node, kind] = [texts, numbers, starts, counts, labels, targets, bindings, node_2, kind_2];
          continue;
        }
      }
    } else {
      return ({ $: "ml_temporary_readRecordTypeOwnerPresence_loop18_done", ["node"]: node, ["kind"]: kind });
    }
  }
}

function source_ml_temporary_readRecordTypeOwnerPresence_loop17(texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position) {
  for (;;) {
    if ((position < pending.length)) {
      const node = readRequiredNumberFact(pending, position);
      const kind = recordTypeFactKind(texts, starts, counts, labels, targets, node);
      const source_ml_source1_s22 = source_ml_temporary_readRecordTypeOwnerPresence_loop18(texts, numbers, starts, counts, labels, targets, bindings, node, kind);
      switch (source_ml_source1_s22.$) {
        case "ml_temporary_readRecordTypeOwnerPresence_loop18_done": {
          const node_2 = source_ml_source1_s22["node"];
          const kind_2 = source_ml_source1_s22["kind"];
          if ((node_2 === owner)) {
            return ({ $: "ml_temporary_readRecordTypeOwnerPresence_loop17_return", ["ml_temporary_result17"]: true });
          } else {
            const visited = false;
            const index = 0;
            const visited_2 = source_ml_temporary_readRecordTypeOwnerPresence_loop19(seen, node_2, visited, index);
            if (visited_2) {
              const position_2 = (position + 1);
              [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position] = [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position_2];
              continue;
            } else {
              const seen_2 = [...seen, ...[node_2]];
              if (((((kind_2 === "array") || (kind_2 === "option")) || (kind_2 === "set")) || (kind_2 === "cell"))) {
                const pending_2 = [...pending, ...[recordTypeFactField(starts, counts, labels, targets, node_2, "element")]];
                const position_3 = (position + 1);
                [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position] = [texts, numbers, starts, counts, labels, targets, bindings, owner, pending_2, seen_2, position_3];
                continue;
              } else {
                if ((kind_2 === "map")) {
                  const pending_3 = [...pending, ...[recordTypeFactField(starts, counts, labels, targets, node_2, "key")]];
                  const pending_4 = [...pending_3, ...[recordTypeFactField(starts, counts, labels, targets, node_2, "value")]];
                  const position_4 = (position + 1);
                  [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position] = [texts, numbers, starts, counts, labels, targets, bindings, owner, pending_4, seen_2, position_4];
                  continue;
                } else {
                  if ((((kind_2 === "tuple") || (kind_2 === "union")) || (kind_2 === "function"))) {
                    const field = ((kind_2 === "tuple") ? "elements" : ((kind_2 === "union") ? "alternatives" : "params"));
                    const sequence = recordTypeFactField(starts, counts, labels, targets, node_2, field);
                    const children = recordTypeFactElements(starts, counts, targets, sequence);
                    const index_2 = 0;
                    const pending_5 = source_ml_temporary_readRecordTypeOwnerPresence_loop20(pending, children, index_2);
                    if ((kind_2 === "function")) {
                      const pending_6 = [...pending_5, ...[recordTypeFactField(starts, counts, labels, targets, node_2, "ret")]];
                      const position_5 = (position + 1);
                      [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position] = [texts, numbers, starts, counts, labels, targets, bindings, owner, pending_6, seen_2, position_5];
                      continue;
                    } else {
                      const position_6 = (position + 1);
                      [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position] = [texts, numbers, starts, counts, labels, targets, bindings, owner, pending_5, seen_2, position_6];
                      continue;
                    }
                  } else {
                    if ((kind_2 === "record")) {
                      const sequence_2 = recordTypeFactField(starts, counts, labels, targets, node_2, "fields");
                      const fields = recordTypeFactElements(starts, counts, targets, sequence_2);
                      const index_3 = 0;
                      const pending_7 = source_ml_temporary_readRecordTypeOwnerPresence_loop21(starts, counts, labels, targets, pending, fields, index_3);
                      const position_7 = (position + 1);
                      [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position] = [texts, numbers, starts, counts, labels, targets, bindings, owner, pending_7, seen_2, position_7];
                      continue;
                    } else {
                      const position_8 = (position + 1);
                      [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position] = [texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen_2, position_8];
                      continue;
                    }
                  }
                }
              }
            }
          }
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s22.$}`);
        }
      }
    } else {
      return ({ $: "ml_temporary_readRecordTypeOwnerPresence_loop17_done" });
    }
  }
}

function source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22(key, value, at) {
  for (;;) {
    if ((at < key.length)) {
      const digit = (Number((key)[at]) - 48);
      if (((!Number.isInteger(digit) || (digit < 0)) || (digit > 9))) {
        return ({ $: "ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_return", ["ml_temporary_result22"]: (-1) });
      } else {
        const value_2 = ((value * 10) + digit);
        const at_2 = (at + 1);
        [key, value, at] = [key, value_2, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done", ["value"]: value });
    }
  }
}

function source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23(names, index) {
  for (;;) {
    if ((index < names.length)) {
      const name = readJavaScriptRecordOrderName(names, index);
      if (!acceptRecordFieldName(name)) {
        return ({ $: "ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return", ["ml_temporary_result23"]: false });
      } else {
        if (((index > 0) && (compareRecordFieldNames(readJavaScriptRecordOrderName(names, (index - 1)), name) >= 0))) {
          return ({ $: "ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return", ["ml_temporary_result23"]: false });
        } else {
          const index_2 = (index + 1);
          [names, index] = [names, index_2];
          continue;
        }
      }
    } else {
      return ({ $: "ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done" });
    }
  }
}

function source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25(order, index, value, previous) {
  for (;;) {
    if ((previous < index)) {
      if ((readJavaScriptRecordOrderPosition(order, previous) === value)) {
        return ({ $: "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_return", ["ml_temporary_result25"]: false });
      } else {
        const previous_2 = (previous + 1);
        [order, index, value, previous] = [order, index, value, previous_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done" });
    }
  }
}

function source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24(count, order, index) {
  for (;;) {
    if ((index < order.length)) {
      const value = readJavaScriptRecordOrderPosition(order, index);
      if ((((!Number.isInteger(value) || (value < 0)) || (value >= count)) || ((value === 0) && ((1 / value) < 0)))) {
        return ({ $: "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return", ["ml_temporary_result24"]: false });
      } else {
        const previous = 0;
        const source_ml_source1_s23 = source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25(order, index, value, previous);
        switch (source_ml_source1_s23.$) {
          case "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done": {
            const index_2 = (index + 1);
            [count, order, index] = [count, order, index_2];
            continue;
          }
          case "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_return": {
            const source_ml_temporary_result25 = source_ml_source1_s23["ml_temporary_result25"];
            return ({ $: "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return", ["ml_temporary_result24"]: source_ml_temporary_result25 });
          }
          default: {
            throw new TypeError(`unexpected constructor ${source_ml_source1_s23.$}`);
          }
        }
      }
    } else {
      return ({ $: "ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done" });
    }
  }
}

function source_ml_temporary_createJavaScriptRecordPropertyOrder_loop27(names, numeric, key, position) {
  for (;;) {
    if (((position < numeric.length) && (readJavaScriptLiteralPropertyIndex(readJavaScriptRecordOrderName(names, readJavaScriptRecordOrderPosition(numeric, position))) < key))) {
      const position_2 = (position + 1);
      [names, numeric, key, position] = [names, numeric, key, position_2];
      continue;
    } else {
      return position;
    }
  }
}

function source_ml_temporary_createJavaScriptRecordPropertyOrder_loop26(names, numeric, field) {
  for (;;) {
    if ((field < names.length)) {
      const key = readJavaScriptLiteralPropertyIndex(readJavaScriptRecordOrderName(names, field));
      if ((key < 0)) {
        const field_2 = (field + 1);
        [names, numeric, field] = [names, numeric, field_2];
        continue;
      } else {
        const position = 0;
        const position_2 = source_ml_temporary_createJavaScriptRecordPropertyOrder_loop27(names, numeric, key, position);
        const numeric_2 = (() => {
          const source_ml_source1_concat_value24 = (numeric).slice(0, position_2);
          const source_ml_source1_concat_value25 = [field];
          const source_ml_source1_concat_value26 = (numeric).slice(position_2, Infinity);
          return [...[...[...[], ...source_ml_source1_concat_value24], ...source_ml_source1_concat_value25], ...source_ml_source1_concat_value26];
        })();
        const field_3 = (field + 1);
        [names, numeric, field] = [names, numeric_2, field_3];
        continue;
      }
    } else {
      return numeric;
    }
  }
}

function source_ml_temporary_createJavaScriptRecordPropertyOrder_loop28(names, insertion, result, index) {
  for (;;) {
    if ((index < insertion.length)) {
      const field = readJavaScriptRecordOrderPosition(insertion, index);
      if ((readJavaScriptLiteralPropertyIndex(readJavaScriptRecordOrderName(names, field)) < 0)) {
        const result_2 = (() => {
          const source_ml_source1_concat_value27 = result;
          const source_ml_source1_concat_value28 = [field];
          return [...[...[], ...source_ml_source1_concat_value27], ...source_ml_source1_concat_value28];
        })();
        const index_2 = (index + 1);
        [names, insertion, result, index] = [names, insertion, result_2, index_2];
        continue;
      } else {
        const index_3 = (index + 1);
        [names, insertion, result, index] = [names, insertion, result, index_3];
        continue;
      }
    } else {
      return result;
    }
  }
}

function source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29(order, expected, index) {
  for (;;) {
    if ((index < order.length)) {
      if ((readJavaScriptRecordOrderPosition(order, index) !== readJavaScriptRecordOrderPosition(expected, index))) {
        return ({ $: "ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_return", ["ml_temporary_result29"]: false });
      } else {
        const index_2 = (index + 1);
        [order, expected, index] = [order, expected, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done" });
    }
  }
}

function source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31(names, insertion, values, index) {
  for (;;) {
    if ((index < values.length)) {
      const field = readJavaScriptRecordOrderPosition(values, index);
      if ((((!Number.isInteger(field) || (field < 0)) || (field >= names.length)) || ((field === 0) && ((1 / field) < 0)))) {
        return ({ $: "ml_temporary_readJavaScriptRecordOrderMergeError_loop31_return", ["ml_temporary_result31"]: "invalid ordered record source field" });
      } else {
        if (!(insertion).includes(field, 0)) {
          const insertion_2 = (() => {
            const source_ml_source1_concat_value29 = insertion;
            const source_ml_source1_concat_value30 = [field];
            return [...[...[], ...source_ml_source1_concat_value29], ...source_ml_source1_concat_value30];
          })();
          const index_2 = (index + 1);
          [names, insertion, values, index] = [names, insertion_2, values, index_2];
          continue;
        } else {
          const index_3 = (index + 1);
          [names, insertion, values, index] = [names, insertion, values, index_3];
          continue;
        }
      }
    } else {
      return ({ $: "ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done", ["insertion"]: insertion });
    }
  }
}

function source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30(names, chunks, insertion, chunk) {
  for (;;) {
    if ((chunk < chunks.length)) {
      const values = readJavaScriptRecordOrderName(chunks, chunk);
      const index = 0;
      const source_ml_source1_s31 = source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31(names, insertion, values, index);
      switch (source_ml_source1_s31.$) {
        case "ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done": {
          const insertion_2 = source_ml_source1_s31["insertion"];
          const chunk_2 = (chunk + 1);
          [names, chunks, insertion, chunk] = [names, chunks, insertion_2, chunk_2];
          continue;
        }
        case "ml_temporary_readJavaScriptRecordOrderMergeError_loop31_return": {
          const source_ml_temporary_result31 = source_ml_source1_s31["ml_temporary_result31"];
          return ({ $: "ml_temporary_readJavaScriptRecordOrderMergeError_loop30_return", ["ml_temporary_result30"]: source_ml_temporary_result31 });
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s31.$}`);
        }
      }
    } else {
      return ({ $: "ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done", ["insertion"]: insertion });
    }
  }
}

function source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop33(insertion, values, index) {
  for (;;) {
    if ((index < values.length)) {
      const field = readJavaScriptRecordOrderPosition(values, index);
      if (!(insertion).includes(field, 0)) {
        const insertion_2 = (() => {
          const source_ml_source1_concat_value32 = insertion;
          const source_ml_source1_concat_value33 = [field];
          return [...[...[], ...source_ml_source1_concat_value32], ...source_ml_source1_concat_value33];
        })();
        const index_2 = (index + 1);
        [insertion, values, index] = [insertion_2, values, index_2];
        continue;
      } else {
        const index_3 = (index + 1);
        [insertion, values, index] = [insertion, values, index_3];
        continue;
      }
    } else {
      return insertion;
    }
  }
}

function source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop32(chunks, insertion, chunk) {
  for (;;) {
    if ((chunk < chunks.length)) {
      const values = readJavaScriptRecordOrderName(chunks, chunk);
      const index = 0;
      const insertion_2 = source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop33(insertion, values, index);
      const chunk_2 = (chunk + 1);
      [chunks, insertion, chunk] = [chunks, insertion_2, chunk_2];
      continue;
    } else {
      return insertion;
    }
  }
}

function source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34(target, source, index) {
  for (;;) {
    if ((index < source.length)) {
      if ((readRecordFieldPosition(target, readJavaScriptRecordOrderName(source, index)) < 0)) {
        return ({ $: "ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_return", ["ml_temporary_result34"]: "an ordered record spread field is missing from its target" });
      } else {
        const index_2 = (index + 1);
        [target, source, index] = [target, source, index_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done" });
    }
  }
}

function source_ml_temporary_rebaseJavaScriptRecordPropertyOrder_loop35(target, source, order, result, index) {
  for (;;) {
    if ((index < order.length)) {
      const name = readJavaScriptRecordOrderName(source, readJavaScriptRecordOrderPosition(order, index));
      const field = readRecordFieldPosition(target, name);
      const result_2 = (() => {
        const source_ml_source1_concat_value34 = result;
        const source_ml_source1_concat_value35 = [field];
        return [...[...[], ...source_ml_source1_concat_value34], ...source_ml_source1_concat_value35];
      })();
      const index_2 = (index + 1);
      [target, source, order, result, index] = [target, source, order, result_2, index_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_readJavaScriptRecordPropertyKeys_loop36(names, order, result, index) {
  for (;;) {
    if ((index < order.length)) {
      const result_2 = (() => {
        const source_ml_source1_concat_value36 = result;
        const source_ml_source1_concat_value37 = [readJavaScriptRecordOrderName(names, readJavaScriptRecordOrderPosition(order, index))];
        return [...[...[], ...source_ml_source1_concat_value36], ...source_ml_source1_concat_value37];
      })();
      const index_2 = (index + 1);
      [names, order, result, index] = [names, order, result_2, index_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop38(units, key, at) {
  for (;;) {
    if ((at < units.length)) {
      const key_2 = (key + (() => {
        const source_ml_source1_character_argument38 = readJavaScriptRecordOrderPosition(units, at);
        return ml_utf16_text.characterUnits([(+(source_ml_source1_character_argument38))]).map(ml_unit=>String.fromCharCode(ml_unit)).join("");
      })());
      const at_2 = (at + 1);
      [units, key, at] = [units, key_2, at_2];
      continue;
    } else {
      return key;
    }
  }
}

function source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop37(rows, result, index) {
  for (;;) {
    if ((index < rows.length)) {
      const units = readJavaScriptRecordOrderName(rows, index);
      const key = "";
      const at = 0;
      const key_2 = source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop38(units, key, at);
      const result_2 = (() => {
        const source_ml_source1_concat_value39 = result;
        const source_ml_source1_concat_value40 = [key_2];
        return [...[...[], ...source_ml_source1_concat_value39], ...source_ml_source1_concat_value40];
      })();
      const index_2 = (index + 1);
      [rows, result, index] = [rows, result_2, index_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_readJavaScriptRecordPropertyIndex_loop39(key, units, index) {
  for (;;) {
    if ((index < key.length)) {
      const units_2 = (() => {
        const source_ml_source1_concat_value50 = units;
        const source_ml_source1_concat_value51 = [(() => {
          const source_ml_source1_string_unit_operand41 = key;
          const source_ml_source1_string_unit_operand42 = index;
          return (((text, position) => (() => {
            const index_2 = (Number.isNaN(position) ? 0 : Math.trunc(position));
            if (((index_2 < 0) || (index_2 >= text.length))) {
              return NaN;
            } else {
              return ((() => {
                const source_ml_source1_string_unit_operand48 = (() => {
                  const source_ml_source1_string_range_operand43 = text;
                  const source_ml_source1_string_range_operand44 = index_2;
                  const source_ml_source1_string_range_operand45 = (index_2 + 1);
                  const source_ml_source1_string_range_position46 = (+(source_ml_source1_string_range_operand44));
                  const source_ml_source1_string_range_position47 = (+(source_ml_source1_string_range_operand45));
                  return (source_ml_source1_string_range_operand43).slice(source_ml_source1_string_range_position46, source_ml_source1_string_range_position47);
                })();
                const source_ml_source1_string_unit_operand49 = 0;
                return (source_ml_source1_string_unit_operand48).codePointAt((+(source_ml_source1_string_unit_operand49)));
              })() ?? NaN);
            }
          })()))(source_ml_source1_string_unit_operand41, (+(source_ml_source1_string_unit_operand42)));
        })()];
        return [...[...[], ...source_ml_source1_concat_value50], ...source_ml_source1_concat_value51];
      })();
      const index_2 = (index + 1);
      [key, units, index] = [key, units_2, index_2];
      continue;
    } else {
      return units;
    }
  }
}

function source_ml_temporary_createJavaScriptDensePropertyKeyTexts_loop40(length, result, index) {
  for (;;) {
    if ((index < length)) {
      const result_2 = (() => {
        const source_ml_source1_concat_value53 = result;
        const source_ml_source1_concat_value54 = [(() => {
          const source_ml_source1_string_argument52 = index;
          return String(source_ml_source1_string_argument52);
        })()];
        return [...[...[], ...source_ml_source1_concat_value53], ...source_ml_source1_concat_value54];
      })();
      const index_2 = (index + 1);
      [length, result, index] = [length, result_2, index_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_createJavaScriptStringOwnPropertyValues_loop41(text, result, index) {
  for (;;) {
    if ((index < text.length)) {
      const result_2 = (() => {
        const source_ml_source1_concat_value62 = result;
        const source_ml_source1_concat_value63 = [(() => {
          const source_ml_source1_string_unit_operand55 = text;
          const source_ml_source1_string_unit_operand56 = index;
          return (((text_2, position) => (() => {
            const index_2 = (Number.isNaN(position) ? 0 : Math.trunc(position));
            if (((index_2 < 0) || (index_2 >= text_2.length))) {
              return "";
            } else {
              const source_ml_source1_string_range_operand57 = text_2;
              const source_ml_source1_string_range_operand58 = index_2;
              const source_ml_source1_string_range_operand59 = (index_2 + 1);
              const source_ml_source1_string_range_position60 = (+(source_ml_source1_string_range_operand58));
              const source_ml_source1_string_range_position61 = (+(source_ml_source1_string_range_operand59));
              return (source_ml_source1_string_range_operand57).slice(source_ml_source1_string_range_position60, source_ml_source1_string_range_position61);
            }
          })()))(source_ml_source1_string_unit_operand55, (+(source_ml_source1_string_unit_operand56)));
        })()];
        return [...[...[], ...source_ml_source1_concat_value62], ...source_ml_source1_concat_value63];
      })();
      const index_2 = (index + 1);
      [text, result, index] = [text, result_2, index_2];
      continue;
    } else {
      return result;
    }
  }
}

function main() {

}

main();

return { create: createJavaScriptRecordPropertyOrder, merge: mergeJavaScriptRecordPropertyOrder, rebase: rebaseJavaScriptRecordPropertyOrder, keys: readJavaScriptRecordPropertyKeys, valid: acceptJavaScriptRecordPropertyOrder };
})();
