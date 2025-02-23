<template>
  <div class="panel">
    <slot />
  </div>
</template>

<style>
.panel {
  background-color: var(--panel-odd);
  border-radius: 5px;
}

.panel .panel {
  background-color: var(--panel-even);
}

.panel .panel .panel {
  background-color: var(--panel-odd);
}

.panel .panel .panel .panel {
  background-color: var(--panel-even);
}
</style>
