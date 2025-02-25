<script lang="ts" setup>
defineProps<{
  options: { value: any; label: string }[];
}>();

const model = defineModel<any>();
</script>

<template>
  <!-- TODO: reka-uiのやつ使う -->
  <select v-model="model">
    <option v-for="option in options" :key="option.value" :value="option.value">{{ option.label }}</option>
  </select>
</template>

<style module>
</style>
