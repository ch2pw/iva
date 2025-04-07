<script lang="ts" setup>
import { ref } from 'vue';
import { itemMeta } from '../item-meta';
import { duration, contains } from '../scripts/timerange-utils';
import { useItemsStore } from '../stores/items';
import { useTimeStore } from '../stores/time';
import { Menu, MenuItem, Submenu } from '@tauri-apps/api/menu';
import { useCursorStore } from '../stores/cursor';

const itemsStore = useItemsStore();
const cursorStore = useCursorStore();
const zoom = ref(0.3);
const timeStore = useTimeStore();

function updateTime(event: MouseEvent) {
  const elementX = (event.currentTarget as HTMLElement).getBoundingClientRect().left;
  timeStore.time = Math.round((event.clientX - elementX) / zoom.value);
}

function outSideMousemove(event: MouseEvent) {
  if (event.buttons === 1) {
    updateTime(event);
    cursorStore.state = 'ew-resize';
  } else {
    cursorStore.state = undefined;
  }
}

function outSideMousedown(event: MouseEvent) {
  updateTime(event);
  if (itemsStore.selectedItem && !contains(itemsStore.selectedItem.props.time, timeStore.time)) {
    itemsStore.selectedItem = null;
  }
}

async function contextmenu(_: MouseEvent, layer: number) {
  const menu = await Menu.new({
    items: [
      await Submenu.new({
        text: "新規アイテム",
        items: [
          ...await Promise.all(Object.entries(itemMeta).map(async ([kind, meta]) => {
            return await MenuItem.new({
              text: meta.name,
              action: () => {
                itemsStore.add({
                  id: crypto.randomUUID(),
                  filters: [],
                  name: meta.name,
                  layer: layer,
                  props: {
                    kind: kind,
                    time: { start: timeStore.time, end: timeStore.time + 1000 },
                    ...Object.fromEntries(Object.entries(meta.propsDefinition).map(([key, value]) => [key, value.default]))
                  },
                });
              },
            });
          })),
        ]
      }),
    ]
  });
  menu.popup();
}
</script>

<template>
  <div :class="$style.container" @mousedown="outSideMousedown" @mousemove.left="outSideMousemove">
    <div :class="$style.time" :style="{ left: timeStore.time * zoom + 'px' }"></div>
    <div :class="$style.ruler">
      <div v-for="i in 100" :key="i" :class="$style.mark" :style="{ width: 500 * zoom + 'px' }">
        {{ (i - 1) * 500 }}
      </div>
    </div>
    <div v-for="i in 50" :key="i" :class="$style.layer" @contextmenu.stop.prevent="contextmenu($event, i)">
      <div v-for="item in itemsStore.layers[i] ?? []" :key="item.id"
        :class="[$style.item, { [$style.selected]: itemsStore.selectedItem === item }]"
        :style="{ left: item.props.time.start * zoom + 'px', width: duration(item.props.time) * zoom + 'px', '--color': itemMeta[item.props.kind].color }"
        @mousedown.stop="itemsStore.selectedItem = item" @contextmenu.stop @mousemove.left.stop>
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

  user-select: none;
  -webkit-user-select: none;
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
  pointer-events: none;
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
  align-items: start;
  justify-content: left;
  height: 1300px;
  padding: 2px 5px;
  border-right: 1px solid var(--divider);
  color: var(--text-transparent);
  user-select: none;
  -webkit-user-select: none;
  cursor: default;
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
