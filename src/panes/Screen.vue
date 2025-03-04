<script setup lang="ts">
import { watch } from 'vue';
import init, { render } from '../../src-wasm/pkg';
import { useItemsStore } from '../stores/items';
import { useTimeStore } from '../stores/time';
import { IconPlayerPauseFilled, IconPlayerPlayFilled } from '@tabler/icons-vue';

const itemsStore = useItemsStore();
const timeStore = useTimeStore();
init().then(() => render(itemsStore.layers, BigInt(timeStore.time)));

watch([() => itemsStore.layers, () => timeStore.time], (v) => {
  render(v[0], BigInt(v[1]));
}, { deep: true });
</script>

<template>
  <div :class="$style.container">
    <canvas id="screen" :class="$style.screen"></canvas>
    <button @click="timeStore.isPlaying = !timeStore.isPlaying" :class="$style.button">
      <IconPlayerPauseFilled v-if="timeStore.isPlaying" />
      <IconPlayerPlayFilled v-else />
    </button>
  </div>
</template>

<style module>
.container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  width: 100%;
  position: relative;
}

.screen {
  max-height: 100%;
  max-width: 100%;
  aspect-ratio: 16 / 9;
  overflow: hidden;
  margin: auto;
}

.button {
  position: absolute;
  height: min-content;
  bottom: 0;
  left: 0;
}
</style>
