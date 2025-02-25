import { PropDefinition } from "./types";

export const itemMeta = {
  rect: {
    name: "四角形",
    color: "#4e7682",
    propsDefinition: {
      x: {
        label: "X",
        type: "number",
        default: 0,
      },
      y: {
        label: "Y",
        type: "number",
        default: 0,
      },
      width: {
        label: "幅",
        type: "slider",
        default: 100,
        min: 0,
        max: 1000,
      },
      height: {
        label: "高さ",
        type: "slider",
        default: 100,
        min: 0,
        max: 1000,
      },
      align: {
        label: "原点",
        type: "select",
        default: "left",
        options: [{
          value: "left-top",
          label: "右上",
        }, {
          value: "center-top",
          label: "中央上",
        }, {
          value: "right-top",
          label: "右上",
        }, {
          value: "left-center",
          label: "左中央",
        }, {
          value: "center",
          label: "中央",
        }, {
          value: "right-center",
          label: "右中央",
        }, {
          value: "left-bottom",
          label: "左下",
        }, {
          value: "center-bottom",
          label: "中央下",
        }, {
          value: "right-bottom",
          label: "右下",
        }],
      },
      color: {
        label: "色",
        type: "color",
        default: "#000",
      },
    },
  },
  circle: {
    name: "円",
    color: "#826a51",
    propsDefinition: {
      x: {
        label: "X",
        type: "number",
        default: 0,
      },
      y: {
        label: "Y",
        type: "number",
        default: 0,
      },
      radius: {
        label: "半径",
        type: "slider",
        default: 50,
        min: 0,
        max: 500,
      },
      color: {
        label: "色",
        type: "color",
        default: "#000",
      },
    },
  },
  text: {
    name: "文字列",
    color: "#62765a",
    propsDefinition: {
      x: {
        label: "X",
        type: "number",
        default: 0,
      },
      y: {
        label: "Y",
        type: "number",
        default: 0,
      },
      text: {
        label: "文章",
        type: "text",
        multiline: true,
        default: "Hello",
      },
      fontSize: {
        label: "フォントサイズ",
        type: "slider",
        default: 16,
        min: 0,
        max: 100,
      },
      color: {
        label: "色",
        type: "color",
        default: "#000",
      },
    },
  },
  image: {
    name: "画像",
    color: "#606f8c",
    propsDefinition: {
      x: {
        label: "X",
        type: "number",
        default: 0,
      },
      y: {
        label: "Y",
        type: "number",
        default: 0,
      },
      width: {
        label: "幅",
        type: "slider",
        default: 100,
        min: 0,
        max: 1000,
      },
      height: {
        label: "高さ",
        type: "slider",
        default: 100,
        min: 0,
        max: 1000,
      },
      src: {
        label: "ファイル",
        type: "file",
      },
    },
  },
  audio: {
    name: "音声",
    color: "#87636e",
    propsDefinition: {
      src: {
        label: "ファイル",
        type: "file",
      },
    },
  },
  // for debug
  unknown: {
    name: "Unknown",
    color: "#000000",
    propsDefinition: {},
  }
} as Record<string, {
  name: string;
  color: string;
  propsDefinition: Record<string, PropDefinition>;
}>;
