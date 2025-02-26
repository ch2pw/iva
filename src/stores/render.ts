import { defineStore } from "pinia";
import { useItemsStore } from "./items";
import { useTimeStore } from "./time";
import { ref, watch } from "vue";
import { invoke } from '@tauri-apps/api/core'

export const useRenderStore = defineStore("render", () => {
  const itemsStore = useItemsStore();
  const time = useTimeStore();
  const rendered = ref("");

  const prevRender = ref(0);

  render();

  watch([() => itemsStore.source, () => time.time], async () => {
    console.log(prevRender.value);
    if (prevRender.value + 100 < Date.now()) {
      prevRender.value = Date.now();
      console.log("Re-render fired");
      await render();
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
