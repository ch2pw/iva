import { TimeRange } from "../types";

export function duration(time: TimeRange): number {
  return time.end - time.start;
}
