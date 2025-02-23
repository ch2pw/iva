<script setup lang="ts">
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from 'reka-ui';
import { itemMeta } from '../item-meta';
import { useItemsStore } from '../stores/items';
import { computed } from 'vue';

const itemsStore = useItemsStore();
const propsDefinition = computed(() => itemsStore.selectedItem ? itemMeta[itemsStore.selectedItem.kind].propsDefinition : {});
</script>

<template>
  <div :class="$style.container">
    <div v-if="itemsStore.selectedItem" v-for="[prop, definition] in Object.entries(propsDefinition)" :key="prop"
      :class="$style.prop">
      <div>{{ definition.label }}</div>

      <input type="range" v-if="definition.type === 'number'" v-model="itemsStore.selectedItem.props[prop]" />

      <SliderRoot v-else-if="definition.type === 'range'" :class="$style.sliderRoot"
        v-model="itemsStore.selectedItem.props[prop]" :min="definition.min" :max="definition.max"
        :step="definition.step" :default-value="definition.default ? [definition.default] : undefined">
        <SliderTrack :class="$style.sliderTrack">
          <SliderRange :class="$style.sliderRange"></SliderRange>
        </SliderTrack>
        <SliderThumb :class="$style.sliderThumb" />
      </SliderRoot>

      <input type="text" v-else-if="definition.type === 'text'" v-model="itemsStore.selectedItem.props[prop]" />

      <input type="color" v-else-if="definition.type === 'color'" :class="$style.color" v-model="itemsStore.selectedItem.props[prop]" />

      <select v-model="itemsStore.selectedItem.props[prop]" v-else-if="definition.type === 'select'">
        <option v-for="option in definition.options" :key="option" :value="option">{{ option }}</option>
      </select>
    </div>
  </div>
</template>

<style module>
.container {
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.prop {
  display: flex;
  flex-direction: column;
  border-bottom: 1px solid var(--divider);
  padding: 5px;
}

.sliderRoot {
  flex-direction: row;
  height: 20px;
  width: 100%;
  position: relative;
  display: flex;
  align-items: center;
}

.sliderTrack {
  height: 5px;
  border-radius: 9999px;
  position: relative;
  flex-grow: 1;
  background-color: var(--divider);
}

.sliderRange {
  height: 100%;
  border-radius: 9999px;
  position: absolute;
  background-color: var(--primary);
}

.sliderThumb {
  display: block;
  width: 10px;
  height: 10px;
  border-radius: 9999px;
  background-color: var(--primary);
}

.color {
  width: 100%;
  height: 30px;
}
</style>
