<script setup lang="ts">
</script>

<template>
  <div :class="$style.screen"></div>
</template>

<style module>
.screen {
  background-color: black;
  display: flex;
  flex-direction: column;
  height: 100%;
  aspect-ratio: 16 / 9;
  overflow: hidden;
  margin: auto;
}
</style>
