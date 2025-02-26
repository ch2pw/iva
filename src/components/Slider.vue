<script lang="ts" setup>
import { SliderRoot, SliderTrack, SliderRange, SliderThumb, TooltipContent, TooltipPortal, TooltipTrigger, TooltipRoot, TooltipProvider } from 'reka-ui';

defineOptions({
  inheritAttrs: false,
})
const props = defineProps<{ defaultValue?: number }>();
const model = defineModel<number>();
if (!model.value) model.value = props.defaultValue ?? 0;
</script>

<template>
  <TooltipProvider :delay-duration="0">
    <SliderRoot :class="$style.sliderRoot" v-bind="$attrs" :model-value="[model!]"
      @update:model-value="model = $event![0]">
      <SliderTrack :class="$style.sliderTrack">
        <SliderRange :class="$style.sliderRange"></SliderRange>
      </SliderTrack>
      <TooltipRoot disable-closing-trigger>
        <TooltipTrigger as-child>
          <SliderThumb :class="$style.sliderThumb" />
        </TooltipTrigger>

        <TooltipPortal>
          <TooltipContent :class="$style.tooltipContent" :side-offset="6">
            {{ model }}
          </TooltipContent>
        </TooltipPortal>
      </TooltipRoot>
    </SliderRoot>
  </TooltipProvider>
</template>

<style module>
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

.tooltipContent {
  z-index: 99999;
  background-color: var(--panel);
  border: solid 1px var(--divider);
  padding: 5px;
  border-radius: 5px;
}
</style>
