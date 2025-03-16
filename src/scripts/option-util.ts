export function map<T, U>(value: T | undefined, proc: (value: T) => U): U | undefined {
  if (value === undefined) {
    return undefined;
  } else {
    return proc(value);
  }
}
