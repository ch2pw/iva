import { defineStore } from "pinia";
import { computed, ref, watch } from "vue";
import { Item } from "../types";

export const useItemsStore = defineStore("items", () => {
  const source = ref({
    "1aea6df4-9bdc-4b7b-b795-96c1346a37ab": {
      layer: 1,
      item: {
        id: "1aea6df4-9bdc-4b7b-b795-96c1346a37ab",
        name: "Item 1",
        position: { x: 0, y: 0 },
        time: { start: 0, end: 200 },
        filters: [],
        kind: "rect",
        props: {
          color: "#ff0000",
          width: 100,
          height: 100,
        },
      },
    },
    "18350b69-4855-45b6-8d80-f3d8b289fccf": {
      layer: 1,
      item: {
        id: "18350b69-4855-45b6-8d80-f3d8b289fccf",
        name: "Item 2",
        position: { x: 0, y: 0 },
        time: { start: 400, end: 600 },
        filters: [],
        kind: "rect",
        props: {
          color: "#ff00ff",
          width: 100,
          height: 100,
        },
      },
    },
    "c7f02e77-1a39-42cf-baf9-499d6ee3f442": {
      layer: 2,
      item: {
        id: "c7f02e77-1a39-42cf-baf9-499d6ee3f442",
        name: "Item 3",
        position: { x: 0, y: 0 },
        time: { start: 100, end: 800 },
        filters: [],
        kind: "rect",
        props: {
          color: "#00ff00",
          width: 100,
          height: 100,
        },
      },
    },
    "f1d4b4c4-2f0b-4b5f-8f4b-7f1f4e0c1b1d": {
      layer: 2,
      item: {
        id: "f1d4b4c4-2f0b-4b5f-8f4b-7f1f4e0c1b1d",
        name: "Item 4",
        position: { x: 0, y: 0 },
        time: { start: 900, end: 2000 },
        filters: [],
        kind: "rect",
        props: {
          color: "#ffff00",
          width: 100,
          height: 100,
        },
      },
    },
    "f1d4b4c4-2f0b-4b5f-8f4b-7f1f4e0c1b1e": {
      layer: 3,
      item: {
        id: "f1d4b4c4-2f0b-4b5f-8f4b-7f1f4e0c1b1e",
        name: "Item 5",
        position: { x: 0, y: 0 },
        time: { start: 300, end: 700 },
        filters: [],
        kind: "rect",
        props: {
          color: "#0000ff",
          width: 100,
          height: 100,
        },
      },
    },
  } as Record<string, { layer: number, item: Item }>);

  const selected = ref<string | null>(null);

  const layers = computed(() => {
    const layers: Record<number, Item[]> = {};
    for (const { layer, item } of Object.values(source.value)) {
      if (!layers[layer]) layers[layer] = [];
      layers[layer].push(item);
    }
    return layers
  });

  const items = computed(() => {
    const items: Record<string, Item> = {};
    for (const { item } of Object.values(source.value)) {
      items[item.id] = item;
    }
    return items;
  });

  const selectedItem = computed({
    get: () => selected.value ? items.value[selected.value] : null,
    set: (item: Item | null) => selected.value = item?.id ?? null,
  });

  function update(item: Item): void {
    source.value[item.id].item = item;
  }

  function add(layer: number, item: Item): void {
    source.value[item.id] = { layer, item };
  }

  function remove(id: string): void {
    delete source.value[id];
  }

  watch(source, (v) => {
    console.log("items", v);
  }, { deep: true });

  return {
    layers,
    items,
    selectedItem,
    update,
    add,
    remove,
  };
});
