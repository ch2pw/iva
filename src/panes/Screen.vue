<script setup lang="ts">
import { watch } from 'vue';
import init, { render } from '../../src-wasm/pkg/src_wasm';
import { useItemsStore } from '../stores/items';
import { useTimeStore } from '../stores/time';

const itemsStore = useItemsStore();
const timeStore = useTimeStore();
init();

watch([() => itemsStore.items, () => timeStore.time], async (v) => {
  console.log("rendering started");
  const start = performance.now();
  await render(v[0], BigInt(0));
  const end = performance.now();
  console.log("rendered in", end - start, "ms");
}, { deep: true });
</script>

<template>
  <canvas id="screen" :class="$style.screen"></canvas>
</template>

<style module>
.screen {
  width: 100%;
  aspect-ratio: 16 / 9;
  overflow: hidden;
  margin: auto;
}
</style>
