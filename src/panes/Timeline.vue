<script lang="ts" setup>
import { itemMeta } from '../item-meta';
import { duration } from '../scripts/duration';
import { useItemsStore } from '../stores/items';

const itemsStore = useItemsStore();
</script>

<template>
  <div :class="$style.container">
    <div :class="$style.ruler">
      <div v-for="i in 50" :key="i" :class="$style.mark" :style="{ left: (i - 1) * 100 + 'px' }">{{ (i - 1) * 100 }}ms
      </div>
    </div>
    <div v-for="i in 50" :key="i" :class="$style.layer">
      <div v-for="item in itemsStore.layers[i] ?? []" :key="item.id" :class="$style.item"
        :style="{ left: item.time.start + 'px', width: duration(item.time) + 'px' }"
        @click="itemsStore.selectedItem = item">
        {{ item.name }}({{ itemMeta[item.kind].name }})
      </div>
    </div>
  </div>
</template>

<style module>
.container {
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.ruler {
  display: flex;
  position: sticky;
  top: 0;
  flex-direction: row;
  height: 25px;
  border-bottom: 1px solid var(--divider);
}

.mark {
  display: flex;
  align-items: center;
  justify-content: left;
  position: absolute;
  height: 25px;
  width: 100px;
  padding: 5px;
  border-right: 1px solid var(--divider);
  color: var(--text-transparent);
}

.layer {
  display: flex;
  position: relative;
  flex-direction: row;
  height: 25px;
  border-bottom: 1px solid var(--divider);
}

.item {
  display: flex;
  align-items: center;
  justify-content: center;
  position: absolute;
  overflow: auto;
  resize: horizontal;
  cursor: pointer;

  height: 25px;
  top: 0;
  background: linear-gradient(135deg, var(--primary), var(--primary-darken-1));
  color: var(--text-on-primary);
  padding: 2px 10px;
}
</style>
