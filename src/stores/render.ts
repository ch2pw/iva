import { defineStore } from "pinia";
import { useItemsStore } from "./items";
import { useTimeStore } from "./time";
import { ref, watch } from "vue";
import { invoke } from '@tauri-apps/api/core';

export const useRenderStore = defineStore("render", () => {
  const itemsStore = useItemsStore();
  const time = useTimeStore();
  const rendered = ref("");
  const isRendering = ref(false);

  render();

  watch([() => itemsStore.source, () => time.time], async () => {
    if (!isRendering.value) {
      console.log("Re-render fired");
      isRendering.value = true;
      await render();
      isRendering.value = false;
      console.log("Re-render complete");
    }
  }, { deep: true });

  async function render() {
    rendered.value = await invoke<string>("render", {
      items: itemsStore.source,
      time: time.time,
    });
  }

  return { rendered };
});
