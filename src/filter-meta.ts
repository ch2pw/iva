import { PropDefinition } from "./types";

export const filterMeta = {
  gaussianBlur: {
    name: "ガウスぼかし",
    propsDefinition: {
      radius: {
        label: "半径",
        type: "number",
        animatable: false,
        default: 10,
      },
    },
  },
} as Record<string, {
  name: string;
  propsDefinition: Record<string, PropDefinition>;
}>;
