export function double(n) {
  return n + n;
}

export function sumTo(n) {
  return n === 0 ? 0 : n + sumTo(n - 1);
}

export async function fetchValue() {
  return 1;
}
