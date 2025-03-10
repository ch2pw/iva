<script setup lang="ts">
import TextField from '../components/inputs/TextField.vue';
import { itemMeta } from '../item-meta';
import { useItemsStore } from '../stores/items';
import { computed } from 'vue';
import Prop from '../components/Prop.vue';

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
        <Prop :definition="definition" v-model="itemsStore.selectedItem.props[prop]" />
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
</style>
