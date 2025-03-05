export type TimeRange = {
  start: number;
  end: number;
}

export type Filter = {
  kind: string;
  name: string;
  props: Record<string, any>;
};

export type Item = {
  id: string;
  layer: number;
  kind: string;
  name: string;
  time: TimeRange;
  filters: Filter[];
  props: Record<string, any>;
};

export type PropDefinition = (
  {
    type: "slider",
    default?: number,
    min: number,
    max: number,
    step?: number,
  } | {
    type: "animatable-slider",
    default?: number[],
    min: number,
    max: number,
    step?: number,
  } | {
    type: "number",
    default?: number,
    min?: number,
    max?: number,
    step?: number,
  } | {
    type: "animatable-number",
    default?: number[],
    min?: number,
    max?: number,
    step?: number,
  } | {
    type: "color",
    default?: string,
  } | {
    type: "animatable-color",
    default?: string[],
  } | {
    type: "text",
    multiline?: boolean,
    default?: string,
  } | {
    type: "select",
    options: {
      value: string,
      label: string,
    }[],
    default?: string,
  } | {
    type: "file",
    default?: string,
  }
) & {
  label: string,
};
