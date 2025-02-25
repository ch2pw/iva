<script setup lang="ts">
import { SplitterGroup, SplitterPanel, SplitterResizeHandle } from 'reka-ui';
import Panel from './components/Panel.vue';
import Props from './panes/Props.vue';
import Screen from './panes/Screen.vue';
import FileManager from './panes/FileManager.vue';
import Timeline from './panes/Timeline.vue';
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
      <SplitterPanel :class="$style.pane" :as="Panel" :default-size="40">
        <Timeline />
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
  overflow: auto !important; /* disable `overflow: hidden` by reka-ui */
  border: solid 1px var(--divider);
}

.handleH {
  width: 10px;
}

.handleV {
  height: 10px;
}
</style>
