export type Position = {
  x: number;
  y: number;
};

export type TimeRange = {
  start: number;
  end: number;
}

export type Filter = {
  name: string;

  kind: string;
  props: Record<string, any>;
};

export type Item = {
  id: string;
  name: string;
  position: Position;
  time: TimeRange;

  filters: Filter[];

  kind: string;
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
    type: "number",
    default?: number,
    min?: number,
    max?: number,
    step?: number,
  } | {
    type: "color",
  } | {
    type: "text",
    multiline?: boolean,
    default?: string,
  } | {
    type: "select",
    options: string[],
    default?: string,
  }
) & {
  label: string,
};
