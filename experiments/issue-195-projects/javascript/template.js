export function html(strings, ...values) {
  return strings.raw.map((text, index) => `${text}${values[index] ?? ''}`).join('');
}
