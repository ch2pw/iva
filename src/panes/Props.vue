<script setup lang="ts">
import NumberField from '../components/NumberField.vue';
import Slider from '../components/Slider.vue';
import TextField from '../components/TextField.vue';
import Select from '../components/Select.vue';
import { itemMeta } from '../item-meta';
import { useItemsStore } from '../stores/items';
import { computed } from 'vue';
import { IconKeyframes } from '@tabler/icons-vue';

const itemsStore = useItemsStore();
const propsDefinition = computed(() => itemsStore.selectedItem ? itemMeta[itemsStore.selectedItem.kind].propsDefinition : {});
</script>

<template>
  <div :class="$style.container">
    <template v-if="itemsStore.selectedItem">
      <div :class="$style.title">{{ itemMeta[itemsStore.selectedItem.kind].name }}</div>
      <div :class="$style.prop">
        <div>名前</div>
        <TextField v-model="itemsStore.selectedItem.name" />
      </div>
      <div v-for="[prop, definition] in Object.entries(propsDefinition)" :key="prop" :class="$style.prop">
        <div :class="$style.header">
          <span>{{ definition.label }}</span>
          <button v-if="definition.animatable" :class="$style.button" @click="itemsStore.selectedItem.props[prop].push({
            progress: 1.0,
            value: definition.newDefault,
          })">
            <IconKeyframes :size="20" :stroke-width="2" />
          </button>
        </div>

        <NumberField v-if="definition.type === 'number' && !definition.animatable"
          v-model="itemsStore.selectedItem.props[prop]" />

        <template v-else-if="definition.type === 'number' && definition.animatable">
          <div v-for="i in itemsStore.selectedItem.props[prop].length" :key="i" :class="$style.controlPoint">
            <NumberField v-model="itemsStore.selectedItem.props[prop][i - 1].value" />
            <NumberField :class="$style.progress" v-if="1 < itemsStore.selectedItem.props[prop].length"
              v-model="itemsStore.selectedItem.props[prop][i - 1].progress" :min="0.0" :max="1.0" :step="0.01"
              :buttons="false" />
          </div>
        </template>

        <Slider v-else-if="definition.type === 'slider' && !definition.animatable"
          v-model="itemsStore.selectedItem.props[prop]" :min="definition.min" :max="definition.max"
          :step="definition.step" />

        <template v-else-if="definition.type === 'slider' && definition.animatable">
          <div v-for="i in itemsStore.selectedItem.props[prop].length" :key="i" :class="$style.controlPoint">
            <Slider v-model="itemsStore.selectedItem.props[prop][i - 1].value" :min="definition.min"
              :max="definition.max" :step="definition.step" />
            <NumberField :class="$style.progress" v-if="1 < itemsStore.selectedItem.props[prop].length"
              v-model="itemsStore.selectedItem.props[prop][i - 1].progress" :min="0.0" :max="1.0" :step="0.01"
              :buttons="false" />
          </div>
        </template>

        <TextField v-else-if="definition.type === 'text'" v-model="itemsStore.selectedItem.props[prop]"
          :multiline="definition.multiline" />

        <input type="color" v-else-if="definition.type === 'color'" :class="$style.color"
          v-model="itemsStore.selectedItem.props[prop]" />

        <Select v-else-if="definition.type === 'select'" v-model="itemsStore.selectedItem.props[prop]"
          :options="definition.options" />
      </div>
    </template>
  </div>
</template>

<style module>
.container {
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.title {
  font-size: var(--font-size-large);
  font-weight: bold;
  padding: 5px;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.prop {
  display: flex;
  flex-direction: column;
  border-bottom: 1px solid var(--divider);
  gap: 5px;
  padding: 10px;
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
