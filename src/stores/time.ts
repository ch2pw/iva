import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { ref, watch } from "vue";

export const useTimeStore = defineStore("time", () => {
  const time = ref(0);
  const isPlaying = ref(false);

  setInterval(() => {
    if (isPlaying.value) {
      time.value += 30;
    }
  }, 30);

  invoke("update_time", { time: time.value });
  watch(time, () => {
    invoke("update_time", { time: time.value });
  });

  return { time, isPlaying };
});
