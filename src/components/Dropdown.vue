<template>
  <div class="relative" ref="dropdownRef">
    <button @click="toggle" class="flex items-center p-2 rounded-md hover:bg-gray-700 focus:outline-none">
      <slot name="trigger"></slot>
    </button>
    <div 
      v-if="isOpen" 
      class="absolute mt-2 w-48 bg-gray-800 border border-gray-700 rounded-md shadow-lg z-10"
      :class="menuAlign === 'right' ? 'right-0' : 'left-0'"
    >
      <slot name="content"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

defineProps({
  menuAlign: {
    type: String,
    default: 'left',
  },
});

const isOpen = ref(false);
const dropdownRef = ref<HTMLElement | null>(null);

const toggle = () => {
  isOpen.value = !isOpen.value;
};

const handleClickOutside = (event: MouseEvent) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    isOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener('mousedown', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('mousedown', handleClickOutside);
});
</script>