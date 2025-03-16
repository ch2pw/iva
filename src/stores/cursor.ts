import { defineStore } from "pinia";
import { ref } from "vue";
import type { Property } from "csstype";

export const useCursorStore = defineStore("cursor", () => {
  const state = ref<Property.Cursor | undefined>();

  return {
    state,
  };
});
