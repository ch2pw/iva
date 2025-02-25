<script setup lang="ts">
import { IconMinus, IconPlus } from '@tabler/icons-vue';
import { NumberFieldDecrement, NumberFieldIncrement, NumberFieldInput, NumberFieldRoot } from 'reka-ui';

</script>

<template>
  <NumberFieldRoot :class="$style.numberFieldRoot">
    <NumberFieldDecrement :class="$style.numberFieldDecrement">
      <IconMinus />
    </NumberFieldDecrement>
    <NumberFieldInput :class="$style.numberFieldInput" />
    <NumberFieldIncrement :class="$style.numberFieldIncrement">
      <IconPlus />
    </NumberFieldIncrement>
  </NumberFieldRoot>
</template>

<style module>
.numberFieldRoot {
  display: flex;
  align-items: center;
  gap: 5px;
  width: 100%;
}

.numberFieldInput {
  flex: 1;
  min-width: 0;
  height: 20px;
  border: none;
  border-radius: 5px;
  padding: 0 5px;
  text-align: center;
  background-color: var(--divider);
  color: var(--text);
}

.numberFieldDecrement,
.numberFieldIncrement {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 5px;
  background-color: var(--primary);
  color: white;
}

.numberFieldDecrement:hover,
.numberFieldIncrement:hover {
  background-color: var(--primary-darken-1);
}
</style>
