<template>
  <aside class="bg-gray-800 text-white flex flex-col transition-all duration-300 ease-in-out"
    :class="isExpanded ? 'w-64' : 'w-16'">
    <div class="p-4 text-2xl font-bold border-b border-gray-700 h-20 flex items-center justify-between">
      <span v-if="isExpanded">GPT on a Budget</span>
      <button @click="$emit('toggle-sidebar')" class="p-2 rounded-md hover:bg-gray-700 focus:outline-none">
        <Menu v-if="isExpanded" class="w-6 h-6" />
        <Menu v-else class="w-6 h-6" />
      </button>
    </div>
    <nav class="flex-1 space-y-2 p-4 overflow-hidden">
      <a href="#" class="flex items-center py-2 px-4 rounded hover:bg-gray-700">
        <Plus class="w-5 h-5 mr-3" />
        <span v-if="isExpanded">New Chat</span>
      </a>
      <!-- Future chat history or other navigation items -->
      <a v-if="isExpanded" v-for="i in 5" :key="i" href="#" class="block py-2 px-4 rounded hover:bg-gray-700 truncate">
        Conversation Title {{ i }}
      </a>
    </nav>
    <div class="p-4 border-t border-gray-700 h-20 flex items-center">
      <a href="#" class="flex items-center py-2 px-4 rounded hover:bg-gray-700">
        <Settings class="w-5 h-5 mr-3" />
        <span v-if="isExpanded">Settings</span>
      </a>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { Plus, Settings, Menu } from 'lucide-vue-next';

defineProps({
  isExpanded: {
    type: Boolean,
    default: true,
  },
});

defineEmits(['toggle-sidebar']);
</script>

<style scoped>
/* Scoped styles for sidebar */
</style>

