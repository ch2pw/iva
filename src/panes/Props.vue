<script setup lang="ts">
import Slider from '../components/Slider.vue';
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

      <input type="number" v-if="definition.type === 'number'" v-model="itemsStore.selectedItem.props[prop]" />

      <Slider v-else-if="definition.type === 'slider'" :model-value="[itemsStore.selectedItem.props[prop]]"
        @update:model-value="value => itemsStore.selectedItem!.props[prop] = value?.[0]" :min="definition.min"
        :max="definition.max" :step="definition.step"
        :default-value="definition.default ? [definition.default] : undefined" />

      <input type="text" v-else-if="definition.type === 'text'" v-model="itemsStore.selectedItem.props[prop]" />

      <input type="color" v-else-if="definition.type === 'color'" :class="$style.color"
        v-model="itemsStore.selectedItem.props[prop]" />

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

.color {
  width: 100%;
  height: 30px;
}
</style>
