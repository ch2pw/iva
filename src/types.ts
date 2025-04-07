export type TimeRange = {
  start: number;
  end: number;
}

export type FilterProp = {
  kind: string;
} & Record<string, any>;

export type ItemProp = {
  kind: string;
  time: TimeRange;
} & Record<string, any>;

export type Item = {
  id: string;
  layer: number;
  name: string;
  filters: {
    name: string;
    props: FilterProp;
  }[];
  props: ItemProp;
};

export type PropDefinition = (
  {
    type: "slider",
    animatable: false,
    default: number,
    min: number,
    max: number,
    step?: number,
  } | {
    type: "slider",
    animatable: true,
    default: {
      progress: number,
      value: number,
    }[],
    newDefault: number,
    min: number,
    max: number,
    step?: number,
  } | {
    type: "number",
    animatable: false,
    default: number,
    min?: number,
    max?: number,
    step?: number,
  } | {
    type: "number",
    animatable: true,
    default: {
      progress: number,
      value: number,
    }[],
    newDefault: number,
    min?: number,
    max?: number,
    step?: number,
  } | {
    type: "color",
    animatable: false,
    default: string,
  } | {
    type: "color",
    animatable: true,
    default: {
      progress: number,
      value: string,
    }[],
    newDefault: number,
  } | {
    type: "text",
    animatable: false,
    default: string,
    multiline?: boolean,
  } | {
    type: "select",
    animatable: false,
    default: string,
    options: {
      value: string,
      label: string,
    }[],
  } | {
    type: "select",
    animatable: true,
    default: {
      progress: number,
      value: string,
    }[],
    newDefault: string,
    options: {
      value: string,
      label: string,
    }[],
  } | {
    type: "file",
    animatable: false,
    default: string,
  }
) & {
  label: string,
};
