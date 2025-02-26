import { defineStore } from "pinia";
import { computed, ref, watch } from "vue";
import { Item } from "../types";

export const useItemsStore = defineStore("items", () => {
  const source = ref({
    "1aea6df4-9bdc-4b7b-b795-96c1346a37ab": {
      id: "1aea6df4-9bdc-4b7b-b795-96c1346a37ab",
      layer: 1,
      name: "Item 1",
      time: { start: 0, end: 200 },
      filters: [],
      kind: "rect",
      props: {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
        align: "center",
        color: "#4e7682",
      },
    },
  } as Record<string, Item>);

  const selected = ref<string | null>(null);

  const layers = computed(() => {
    const layers: Record<number, Item[]> = {};
    for (const item of Object.values(source.value)) {
      if (!layers[item.layer]) layers[item.layer] = [];
      layers[item.layer].push(item);
    }
    return layers
  });

  const items = computed(() => {
    const items: Record<string, Item> = {};
    for (const item of Object.values(source.value)) {
      items[item.id] = item;
    }
    return items;
  });

  const selectedItem = computed({
    get: () => selected.value ? items.value[selected.value] : null,
    set: (item: Item | null) => selected.value = item?.id ?? null,
  });

  function update(item: Item): void {
    source.value[item.id] = item;
  }

  function add(item: Item): void {
    source.value[item.id] = item;
  }

  function remove(id: string): void {
    delete source.value[id];
  }

  watch(source, (v) => {
    console.log("items", v);
  }, { deep: true });

  return {
    source,
    layers,
    items,
    selectedItem,
    update,
    add,
    remove,
  };
});
