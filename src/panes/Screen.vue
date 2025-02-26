<script setup lang="ts">
import { useRenderStore } from '../stores/render';

const renderStore = useRenderStore();
</script>

<template>
  <img :class="$style.screen" :src="`data:image/png;base64,${renderStore.rendered}`" />
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
