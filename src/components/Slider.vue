<script lang="ts" setup>
import { SliderRoot, SliderTrack, SliderRange, SliderThumb, SliderRootProps, SliderRootEmits } from 'reka-ui';

defineOptions({
  inheritAttrs: false,
})
const props = defineProps<SliderRootProps>();
const emit = defineEmits<SliderRootEmits>();
</script>

<template>
  <SliderRoot :class="$style.sliderRoot" v-bind="props" v-on="{
    'update:modelValue': (v: number[]) => emit('update:modelValue', v),
    'valueCommit': (v: number[]) => emit('valueCommit', v),
  }">
    <SliderTrack :class="$style.sliderTrack">
      <SliderRange :class="$style.sliderRange"></SliderRange>
    </SliderTrack>
    <SliderThumb :class="$style.sliderThumb" />
  </SliderRoot>
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
</style>
