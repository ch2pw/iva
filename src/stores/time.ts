import { defineStore } from "pinia";
import { ref } from "vue";

export const useTimeStore = defineStore("time", () => {
  const time = ref(0);
  const isPlaying = ref(false);

  setInterval(() => {
    if (isPlaying.value) {
      time.value += 30;
    }
  }, 30);

  return { time, isPlaying };
});
