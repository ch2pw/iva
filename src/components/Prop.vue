<script lang="ts" setup>
import { IconKeyframes } from '@tabler/icons-vue';
import { PropDefinition } from '../types';
import NumberField from './inputs/NumberField.vue';
import Slider from './inputs/Slider.vue';
import TextField from './inputs/TextField.vue';
import Select from './inputs/Select.vue';

defineProps<{
  definition: PropDefinition;
}>();
const model = defineModel<any>();
</script>

<template>
  <div :class="$style.header">
    <span>{{ definition.label }}</span>
    <button v-if="definition.animatable" :class="$style.button" @click="model.push({
      progress: 1.0,
      value: definition.newDefault,
    })">
      <IconKeyframes :size="20" :stroke-width="2" />
    </button>
  </div>

  <NumberField v-if="definition.type === 'number' && !definition.animatable" v-model="model" />

  <template v-else-if="definition.type === 'number' && definition.animatable">
    <div v-for="i in model.length" :key="i" :class="$style.controlPoint">
      <NumberField v-model="model[i - 1].value" />
      <NumberField :class="$style.progress" v-if="1 < model.length" v-model="model[i - 1].progress"
        :min="model[i - 1 - 1]?.progress ?? 0.0" :max="model[i - 1 + 1]?.progress ?? 1.0" :step="0.01" :buttons="false"
        :format-options="{ style: 'percent' }" />
    </div>
  </template>

  <Slider v-else-if="definition.type === 'slider' && !definition.animatable" v-model="model" :min="definition.min"
    :max="definition.max" :step="definition.step" />

  <template v-else-if="definition.type === 'slider' && definition.animatable">
    <div v-for="i in model.length" :key="i" :class="$style.controlPoint">
      <Slider v-model="model[i - 1].value" :min="definition.min" :max="definition.max" :step="definition.step" />
      <NumberField :class="$style.progress" v-if="1 < model.length" v-model="model[i - 1].progress"
        :min="model[i - 1 - 1]?.progress ?? 0.0" :max="model[i - 1 + 1]?.progress ?? 1.0" :step="0.01" :buttons="false"
        :format-options="{ style: 'percent' }" />
    </div>
  </template>

  <TextField v-else-if="definition.type === 'text'" v-model="model" :multiline="definition.multiline" />

  <input type="color" v-else-if="definition.type === 'color'" :class="$style.color" v-model="model" />

  <Select v-else-if="definition.type === 'select'" v-model="model" :options="definition.options" />
</template>

<style module>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.controlPoint {
  display: flex;
  align-items: center;
  width: 100%;
  gap: 5px;
}

.progress {
  width: 100px;
}

.color {
  width: 100%;
  height: 30px;
}

.button {
  border-radius: 5px;
  padding: 5px;
  display: flex;
  background-color: var(--button);
}

.button:hover {
  background-color: var(--button-hover);
}
</style>
