<script lang="ts" setup>
import { ref } from 'vue';
import { itemMeta } from '../item-meta';
import { duration, contains } from '../scripts/timerange-utils';
import { useItemsStore } from '../stores/items';
import { useTimeStore } from '../stores/time';

const itemsStore = useItemsStore();
const zoom = ref(0.5);
const timeStore = useTimeStore();

function outSideClick(event: MouseEvent) {
  const elementX = (event.currentTarget as HTMLElement).getBoundingClientRect().left;
  timeStore.time = (event.clientX - elementX) / zoom.value;
  if (itemsStore.selectedItem && !contains(itemsStore.selectedItem.time, timeStore.time)) {
    itemsStore.selectedItem = null;
  }
}
</script>

<template>
  <div :class="$style.container" @click="outSideClick">
    <div :class="$style.time" :style="{ left: timeStore.time * zoom + 'px' }"></div>
    <div :class="$style.ruler">
      <div v-for="i in 100" :key="i" :class="$style.mark" :style="{ width: 100 * zoom + 'px' }">
        {{ (i - 1) * 100 }}
      </div>
    </div>
    <div v-for="i in 50" :key="i" :class="$style.layer">
      <div v-for="item in itemsStore.layers[i] ?? []" :key="item.id"
        :class="[$style.item, { [$style.selected]: itemsStore.selectedItem === item }]"
        :style="{ left: item.time.start * zoom + 'px', width: duration(item.time) * zoom + 'px', '--color': itemMeta[item.kind].color }"
        @click.stop="itemsStore.selectedItem = item">
        {{ item.name }}
      </div>
    </div>
  </div>
</template>

<style module>
.container {
  display: flex;
  position: relative;
  flex-direction: column;
  width: fit-content;
  overflow: auto;
}

.time {
  display: flex;
  position: absolute;
  top: 0;
  flex-direction: column;
  width: 1px;
  height: 100%;
  z-index: 9999;
  border-left: 1px solid red;
}

.ruler {
  display: flex;
  position: relative;
  top: 0;
  flex-direction: row;
  width: fit-content;
  height: 25px;
  border-bottom: 1px solid var(--divider);
}

.mark {
  display: flex;
  align-items: center;
  justify-content: left;
  height: 25px;
  padding: 5px;
  border-right: 1px solid var(--divider);
  color: var(--text-transparent);
}

.layer {
  display: flex;
  position: relative;
  flex-direction: row;
  width: 100%;
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
  --color-1: var(--color);
  --color-2: color-mix(in oklch, var(--color-1), black 30%);
  --color-3: color-mix(in oklch, var(--color-1), black 60%);
  background: linear-gradient(135deg, var(--color-1), var(--color-2));
  color: var(--text-on-primary);
  padding: 2px 10px;
}

.selected {
  background: linear-gradient(135deg, var(--color-2), var(--color-3));
}
</style>
