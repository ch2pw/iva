<script setup lang="ts">
import { IconMinus, IconPlus } from '@tabler/icons-vue';
import { NumberFieldDecrement, NumberFieldIncrement, NumberFieldInput, NumberFieldRoot } from 'reka-ui';

</script>

<template>
  <NumberFieldRoot :class="$style.numberFieldRoot">
    <NumberFieldDecrement :class="$style.numberFieldDecrement">
      <IconMinus :size="13" />
    </NumberFieldDecrement>
    <NumberFieldInput :class="$style.numberFieldInput" />
    <NumberFieldIncrement :class="$style.numberFieldIncrement">
      <IconPlus :size="13" />
    </NumberFieldIncrement>
  </NumberFieldRoot>
</template>

<style module>
.numberFieldRoot {
  display: flex;
  align-items: stretch;
  gap: 5px;
  width: 100%;
}

.numberFieldInput {
  flex: 1;
  min-width: 0;
  border: none;
  border-radius: 5px;
  padding: 5px;
  text-align: center;
  border: solid 1px var(--divider);
  color: var(--text);
}

.numberFieldDecrement,
.numberFieldIncrement {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 5px;
  border-radius: 5px;
  background-color: var(--button);
  color: var(--text);
}

.numberFieldDecrement:hover,
.numberFieldIncrement:hover {
  background-color: var(--button-hover);
}
</style>
