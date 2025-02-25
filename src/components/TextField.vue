<script lang="ts" setup>
defineProps<{
  multiline?: boolean;
}>();
defineOptions({
  inheritAttrs: false,
})
const model = defineModel<string>();
</script>

<template>
  <textarea :class="$style.textarea" v-if="multiline" v-bind="$attrs" v-model="model" />
  <input type="text" v-else v-bind="$attrs" v-model="model" />
</template>

<style module>
.textarea {
  padding: 5px;
  resize: vertical;
  min-height: 100px;
  width: 100%;
}
</style>
