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

function matchUtf16Text(text, search, start) {
  if (((start < 0) || ((start + search.length) > text.length))) {
    return false;
  } else {
    const at = 0;
    const source_ml_source1_s3 = source_ml_temporary_matchUtf16Text_loop7(text, search, start, at);
    switch (source_ml_source1_s3.$) {
      case "ml_temporary_matchUtf16Text_loop7_done": {
        return true;
      }
      case "ml_temporary_matchUtf16Text_loop7_return": {
        const source_ml_temporary_result7 = source_ml_source1_s3["ml_temporary_result7"];
        return source_ml_temporary_result7;
      }
      default: {
        throw new TypeError(`unexpected constructor ${source_ml_source1_s3.$}`);
      }
    }
  }
}

function indexUtf16Text(text, search, position) {
  const start = Math.min(Math.max(readUtf16TextInteger(position), 0), text.length);
  const at = start;
  const source_ml_source1_s4 = source_ml_temporary_indexUtf16Text_loop8(text, search, at);
  switch (source_ml_source1_s4.$) {
    case "ml_temporary_indexUtf16Text_loop8_done": {
      return (-1);
    }
    case "ml_temporary_indexUtf16Text_loop8_return": {
      const source_ml_temporary_result8 = source_ml_source1_s4["ml_temporary_result8"];
      return source_ml_temporary_result8;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s4.$}`);
    }
  }
}

function lastIndexUtf16Text(text, search, position) {
  const integer = (Number.isNaN(position) ? Infinity : readUtf16TextInteger(position));
  const start = Math.min(Math.max(integer, 0), (text.length - search.length));
  const at = start;
  const source_ml_source1_s5 = source_ml_temporary_lastIndexUtf16Text_loop9(text, search, at);
  switch (source_ml_source1_s5.$) {
    case "ml_temporary_lastIndexUtf16Text_loop9_done": {
      return (-1);
    }
    case "ml_temporary_lastIndexUtf16Text_loop9_return": {
      const source_ml_temporary_result9 = source_ml_source1_s5["ml_temporary_result9"];
      return source_ml_temporary_result9;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s5.$}`);
    }
  }
}

function compareUtf16Text(left, right) {
  const count = Math.min(left.length, right.length);
  const at = 0;
  const source_ml_source1_s6 = source_ml_temporary_compareUtf16Text_loop10(left, right, count, at);
  switch (source_ml_source1_s6.$) {
    case "ml_temporary_compareUtf16Text_loop10_done": {
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
    case "ml_temporary_compareUtf16Text_loop10_return": {
      const source_ml_temporary_result10 = source_ml_source1_s6["ml_temporary_result10"];
      return source_ml_temporary_result10;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s6.$}`);
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
  const result_2 = source_ml_temporary_joinUtf16Text_loop11(values, separator, result, at);
  return result_2;
}

function trimUtf16Text(text, start, end) {
  const first = 0;
  const last = text.length;
  const first_3 = (start ? (() => {
    const first_2 = source_ml_temporary_trimUtf16Text_loop14(text, first, last);
    return first_2;
  })() : first);
  const last_3 = (end ? (() => {
    const last_2 = source_ml_temporary_trimUtf16Text_loop15(text, first_3, last);
    return last_2;
  })() : last);
  return sliceUtf16Text(text, first_3, last_3);
}

function characterUnitsUtf16Text(values) {
  const result = [];
  const at = 0;
  const result_2 = source_ml_temporary_characterUnitsUtf16Text_loop16(values, result, at);
  return result_2;
}

function codePointUnitsUtf16Text(values) {
  const result = [(-1)];
  const at = 0;
  const source_ml_source1_s7 = source_ml_temporary_codePointUnitsUtf16Text_loop17(values, result, at);
  switch (source_ml_source1_s7.$) {
    case "ml_temporary_codePointUnitsUtf16Text_loop17_done": {
      const result_2 = source_ml_source1_s7["result"];
      return result_2;
    }
    case "ml_temporary_codePointUnitsUtf16Text_loop17_return": {
      const source_ml_temporary_result17 = source_ml_source1_s7["ml_temporary_result17"];
      return source_ml_temporary_result17;
    }
    default: {
      throw new TypeError(`unexpected constructor ${source_ml_source1_s7.$}`);
    }
  }
}

function readUtf8TextUnits(bytes) {
  return readJavaScriptUtf8StringUnits(bytes);
}

function displayPointsUtf16Text(text) {
  const result = [];
  const at = 0;
  const result_2 = source_ml_temporary_displayPointsUtf16Text_loop18(text, result, at);
  return result_2;
}

function displayUtf8FromUtf16Text(text) {
  const bytes = [];
  const points = displayPointsUtf16Text(text);
  const at = 0;
  const bytes_2 = source_ml_temporary_displayUtf8FromUtf16Text_loop20(bytes, points, at);
  return bytes_2;
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
      const source_ml_source1_s8 = (((scalar >= 194) && (scalar <= 223)) ? (() => {
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
      switch (source_ml_source1_s8.$) {
        case "ml_temporary_readJavaScriptUtf8StringUnits_join3_next": {
          const scalar_5 = source_ml_source1_s8["scalar"];
          const remaining_5 = source_ml_source1_s8["remaining"];
          const minimum_5 = source_ml_source1_s8["minimum"];
          const source_ml_source1_s9 = source_ml_temporary_readJavaScriptUtf8StringUnits_loop4(bytes, index_2, scalar_5, remaining_5);
          switch (source_ml_source1_s9.$) {
            case "ml_temporary_readJavaScriptUtf8StringUnits_loop4_done": {
              const index_3 = source_ml_source1_s9["index"];
              const scalar_6 = source_ml_source1_s9["scalar"];
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
              const source_ml_temporary_result4 = source_ml_source1_s9["ml_temporary_result4"];
              return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_temporary_result2"]: source_ml_temporary_result4 });
            }
            default: {
              throw new TypeError(`unexpected constructor ${source_ml_source1_s9.$}`);
            }
          }
        }
        case "ml_temporary_readJavaScriptUtf8StringUnits_join3_return": {
          const source_ml_temporary_result4_2 = source_ml_source1_s8["ml_temporary_result4"];
          return ({ $: "ml_temporary_readJavaScriptUtf8StringUnits_loop2_return", ["ml_temporary_result2"]: source_ml_temporary_result4_2 });
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s8.$}`);
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

function source_ml_temporary_matchUtf16Text_loop7(text, search, start, at) {
  for (;;) {
    if ((at < search.length)) {
      if (((text)[(start + at)] !== (search)[at])) {
        return ({ $: "ml_temporary_matchUtf16Text_loop7_return", ["ml_temporary_result7"]: false });
      } else {
        const at_2 = (at + 1);
        [text, search, start, at] = [text, search, start, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_matchUtf16Text_loop7_done" });
    }
  }
}

function source_ml_temporary_indexUtf16Text_loop8(text, search, at) {
  for (;;) {
    if ((at <= (text.length - search.length))) {
      if (matchUtf16Text(text, search, at)) {
        return ({ $: "ml_temporary_indexUtf16Text_loop8_return", ["ml_temporary_result8"]: at });
      } else {
        const at_2 = (at + 1);
        [text, search, at] = [text, search, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_indexUtf16Text_loop8_done" });
    }
  }
}

function source_ml_temporary_lastIndexUtf16Text_loop9(text, search, at) {
  for (;;) {
    if ((at >= 0)) {
      if (matchUtf16Text(text, search, at)) {
        return ({ $: "ml_temporary_lastIndexUtf16Text_loop9_return", ["ml_temporary_result9"]: at });
      } else {
        const at_2 = (at - 1);
        [text, search, at] = [text, search, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_lastIndexUtf16Text_loop9_done" });
    }
  }
}

function source_ml_temporary_compareUtf16Text_loop10(left, right, count, at) {
  for (;;) {
    if ((at < count)) {
      const a = ((left)[at] ?? 0);
      const b = ((right)[at] ?? 0);
      if ((a !== b)) {
        return ({ $: "ml_temporary_compareUtf16Text_loop10_return", ["ml_temporary_result10"]: ((a < b) ? (-1) : 1) });
      } else {
        const at_2 = (at + 1);
        [left, right, count, at] = [left, right, count, at_2];
        continue;
      }
    } else {
      return ({ $: "ml_temporary_compareUtf16Text_loop10_done" });
    }
  }
}

function source_ml_temporary_joinUtf16Text_loop12(separator, result, offset) {
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

function source_ml_temporary_joinUtf16Text_loop13(result, value, offset) {
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

function source_ml_temporary_joinUtf16Text_loop11(values, separator, result, at) {
  for (;;) {
    if ((at < values.length)) {
      const result_3 = ((at !== 0) ? (() => {
        const offset = 0;
        const result_2 = source_ml_temporary_joinUtf16Text_loop12(separator, result, offset);
        return result_2;
      })() : result);
      const value = ((values)[at] ?? []);
      const offset_2 = 0;
      const result_4 = source_ml_temporary_joinUtf16Text_loop13(result_3, value, offset_2);
      const at_2 = (at + 1);
      [values, separator, result, at] = [values, separator, result_4, at_2];
      continue;
    } else {
      return result;
    }
  }
}

function source_ml_temporary_trimUtf16Text_loop14(text, first, last) {
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

function source_ml_temporary_trimUtf16Text_loop15(text, first, last) {
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

function source_ml_temporary_characterUnitsUtf16Text_loop16(values, result, at) {
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

function source_ml_temporary_codePointUnitsUtf16Text_loop17(values, result, at) {
  for (;;) {
    if ((at < values.length)) {
      const value = ((values)[at] ?? (-1));
      if (((!Number.isInteger(value) || (value < 0)) || (value > 1114111))) {
        return ({ $: "ml_temporary_codePointUnitsUtf16Text_loop17_return", ["ml_temporary_result17"]: [at] });
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
      return ({ $: "ml_temporary_codePointUnitsUtf16Text_loop17_done", ["result"]: result });
    }
  }
}

function source_ml_temporary_displayPointsUtf16Text_loop18(text, result, at) {
  for (;;) {
    if ((at < text.length)) {
      const point = ((text)[at] ?? 65533);
      const source_ml_source1_s10 = (((point >= 55296) && (point <= 56319)) ? (() => {
        const low = ((text)[(at + 1)] ?? (-1));
        if (((low >= 56320) && (low <= 57343))) {
          const point_2 = (((65536 + ((point - 55296) * 1024)) + low) - 56320);
          const at_2 = (at + 1);
          return ({ $: "ml_temporary_displayPointsUtf16Text_join19_next", ["at"]: at_2, ["point"]: point_2 });
        } else {
          const point_3 = 65533;
          return ({ $: "ml_temporary_displayPointsUtf16Text_join19_next", ["at"]: at, ["point"]: point_3 });
        }
      })() : (((point >= 56320) && (point <= 57343)) ? (() => {
        const point_4 = 65533;
        return ({ $: "ml_temporary_displayPointsUtf16Text_join19_next", ["at"]: at, ["point"]: point_4 });
      })() : ({ $: "ml_temporary_displayPointsUtf16Text_join19_next", ["at"]: at, ["point"]: point })));
      switch (source_ml_source1_s10.$) {
        case "ml_temporary_displayPointsUtf16Text_join19_next": {
          const at_3 = source_ml_source1_s10["at"];
          const point_5 = source_ml_source1_s10["point"];
          const result_2 = [...result, ...[point_5]];
          const at_4 = (at_3 + 1);
          [text, result, at] = [text, result_2, at_4];
          continue;
        }
        default: {
          throw new TypeError(`unexpected constructor ${source_ml_source1_s10.$}`);
        }
      }
    } else {
      return result;
    }
  }
}

function source_ml_temporary_displayUtf8FromUtf16Text_loop20(bytes, points, at) {
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

function main() {

}

main();

return { slicePosition: readUtf16TextSlicePosition, slice: sliceUtf16Text, substring: substringUtf16Text, character: characterUtf16Text, relativeCharacter: relativeCharacterUtf16Text, codePoint: codePointUtf16Text, points: pointsUtf16Text, index: indexUtf16Text, lastIndex: lastIndexUtf16Text, compare: compareUtf16Text, starts: startsUtf16Text, ends: endsUtf16Text, includes: includesUtf16Text, trim: trimUtf16Text, characterUnits: characterUnitsUtf16Text, codePointUnits: codePointUnitsUtf16Text, join: joinUtf16Text, displayUtf8: displayUtf8FromUtf16Text, displayPoints: displayPointsUtf16Text, fromUtf8: readUtf8TextUnits };
})();
