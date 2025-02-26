import { defineStore } from "pinia";
import { ref, watch } from "vue";

export const useTimeStore = defineStore("time", () => {
  const time = ref(0);

  watch(time, (v) => {
    console.log("Time changed", v);
  });

  return { time };
});
