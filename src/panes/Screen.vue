<script setup lang="ts">
import { computed } from 'vue';
import { useRenderStore } from '../stores/render';

const renderStore = useRenderStore();
const src = computed(() => {
  return `data:image/png;base64,${renderStore.rendered}`;
});
</script>

<template>
  <img :class="$style.screen" :src="src" />
</template>

<style module>
.screen {
  max-width: 100%;
  max-height: 100%;
  aspect-ratio: 16 / 9;
  overflow: hidden;
  margin: auto;
}
</style>
