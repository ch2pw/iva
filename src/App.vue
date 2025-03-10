<script setup lang="ts">
import { SplitterGroup, SplitterPanel, SplitterResizeHandle } from 'reka-ui';
import Panel from './components/Panel.vue';
import Props from './panes/Props.vue';
import Screen from './panes/Screen.vue';
import FileManager from './panes/FileManager.vue';
import Timeline from './panes/Timeline.vue';
import CurveEditor from './panes/CurveEditor.vue';
import { useItemsStore } from './stores/items';
import { Menu } from '@tauri-apps/api/menu';
import { onMounted } from 'vue';

const itemsStore = useItemsStore();

onMounted(async () => {
  const menu = await Menu.new({
    items: [
      {
        id: 'file',
        text: 'ファイル',
        items: [
          {
            text: 'エクスポート',
            items: [
              {
                id: 'export-mp4',
                text: 'MP4エクスポート',
              },
              {
                id: 'export-gif',
                text: "GIFエクスポート",
              },
              {
                id: 'export-png',
                text: '現在のフレームをPNGエクスポート',
              },
            ],
          }
        ],
      }
    ],
  });

  await menu.setAsAppMenu();
})
</script>

<template>
  <main :class="$style.container">
    <SplitterGroup direction="vertical">
      <SplitterPanel :default-size="60">
        <SplitterGroup direction="horizontal">
          <SplitterPanel :class="$style.pane" :as="Panel" :default-size="20">
            <Props />
          </SplitterPanel>
          <SplitterResizeHandle :class="$style.handleH" />
          <SplitterPanel :class="$style.pane" :as="Panel" :default-size="60">
            <Screen />
          </SplitterPanel>
          <SplitterResizeHandle :class="$style.handleH" />
          <SplitterPanel :class="$style.pane" :as="Panel" :default-size="20">
            <FileManager />
          </SplitterPanel>
        </SplitterGroup>
      </SplitterPanel>
      <SplitterResizeHandle :class="$style.handleV" />
      <SplitterPanel :default-size="40">
        <SplitterGroup direction="horizontal">
          <SplitterPanel :class="$style.pane" :as="Panel">
            <Timeline />
          </SplitterPanel>
          <template v-if="itemsStore.selectedItem">
            <SplitterResizeHandle :class="$style.handleH" />
            <SplitterPanel :class="$style.pane" :as="Panel" :default-size="30">
              <CurveEditor />
            </SplitterPanel>
          </template>
        </SplitterGroup>
      </SplitterPanel>
    </SplitterGroup>
  </main>
</template>

<style module>
.container {
  display: flex;
  width: 100dvw;
  height: 100dvh;
  padding: 10px;
}

.pane {
  overflow: auto !important;
  /* disable `overflow: hidden` by reka-ui */
  border: solid 1px var(--divider);
  position: relative;
}

.handleH {
  width: 10px;
}

.handleV {
  height: 10px;
}
</style>
