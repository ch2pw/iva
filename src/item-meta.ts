import { PropDefinition } from "./types";

export const itemMeta = {
  rect: {
    name: "Rectangle",
    propsDefinition: {
      width: {
        label: "Width",
        type: "range",
        default: 100,
        min: 0,
        max: 1000,
      },
      height: {
        label: "Height",
        type: "range",
        default: 100,
        min: 0,
        max: 1000,
      },
      color: {
        label: "Color",
        type: "color",
        default: "#000",
      },
    },
  },
  none: {
    name: "None",
    propsDefinition: {},
  }
} as Record<string, {
  name: string;
  propsDefinition: Record<string, PropDefinition>;
}>;
