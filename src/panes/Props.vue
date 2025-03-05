<script setup lang="ts">
import NumberField from '../components/NumberField.vue';
import Slider from '../components/Slider.vue';
import TextField from '../components/TextField.vue';
import Select from '../components/Select.vue';
import { itemMeta } from '../item-meta';
import { useItemsStore } from '../stores/items';
import { computed } from 'vue';

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
        <div>{{ definition.label }}</div>

        <NumberField v-if="definition.type === 'number'" v-model="itemsStore.selectedItem.props[prop]" />

        <template v-else-if="definition.type === 'animatable-number'">
          <NumberField v-for="i in itemsStore.selectedItem.props[prop].length" :key="i"
            v-model="itemsStore.selectedItem.props[prop][i - 1]" />
          <button :class="$style.button" @click="itemsStore.selectedItem.props[prop].push(0)">制御点を追加</button>
        </template>

        <Slider v-else-if="definition.type === 'slider'" v-model="itemsStore.selectedItem.props[prop]"
          :min="definition.min" :max="definition.max" :step="definition.step" :default-value="definition.default" />

        <template v-else-if="definition.type === 'animatable-slider'">
          <Slider v-for="i in itemsStore.selectedItem.props[prop].length" :key="i"
            v-model="itemsStore.selectedItem.props[prop][i - 1]" :min="definition.min" :max="definition.max"
            :step="definition.step" :default-value="definition.default?.[i - 1]" />
          <button :class="$style.button" @click="itemsStore.selectedItem.props[prop].push(0)">制御点を追加</button>
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

.prop {
  display: flex;
  flex-direction: column;
  border-bottom: 1px solid var(--divider);
  gap: 5px;
  padding: 10px;
}

.color {
  width: 100%;
  height: 30px;
}

.button {
  border-radius: 5px;
  padding: 5px;
  text-align: center;
  background-color: var(--button);
}

.button:hover {
  background-color: var(--button-hover);
}
</style>
