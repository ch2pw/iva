<script setup lang="ts">
import { onMounted, ref, useTemplateRef, watch } from 'vue';
import { useItemsStore } from '../stores/items';
import { useTimeStore } from '../stores/time';
import { IconPlayerPauseFilled, IconPlayerPlayFilled } from '@tabler/icons-vue';
import { invoke } from '@tauri-apps/api/core';

const canvas = useTemplateRef<HTMLCanvasElement>("screen");
const ctx = ref<CanvasRenderingContext2D | null>(null);

onMounted(() => {
  ctx.value = canvas.value!.getContext("2d");
  canvas.value!.width = 1920;
  canvas.value!.height = 1080;
})

const itemsStore = useItemsStore();
const timeStore = useTimeStore();

let renderRequested = true;
watch([() => itemsStore.layers, () => timeStore.time], () => renderRequested = true, { deep: true });

setInterval(async () => {
  if (renderRequested) {
    renderRequested = false;
    const bytes = await invoke<ArrayBuffer>('render');
    const image = new ImageData(new Uint8ClampedArray(bytes), 1920, 1080);
    ctx.value!.putImageData(image, 0, 0);
  }
}, 30);
</script>

<template>
  <div :class="$style.container">
    <canvas ref="screen" id="screen" :class="$style.screen"></canvas>
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
