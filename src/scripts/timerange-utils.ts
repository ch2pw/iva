import { TimeRange } from "../types";

export function duration(time: TimeRange): number {
  return time.end - time.start;
}

export function inRange(time: TimeRange, t: number): boolean {
  return t >= time.start && t <= time.end;
}
