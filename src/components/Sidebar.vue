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
      <a href="#" class="flex items-center py-2 px-4 rounded hover:bg-gray-700" @click="$emit('new-chat')">
        <Plus class="w-5 h-5 mr-3" />
        <span v-if="isExpanded">New Chat</span>
      </a>
      <!-- Chat history -->
      <a v-if="isExpanded" v-for="conversation in conversationsStore.conversations" :key="conversation.id"
        href="#" class="flex items-center justify-between py-2 px-4 rounded hover:bg-gray-700 truncate"
        :class="{ 'bg-gray-700': conversation.id === conversationsStore.activeConversationId }"
        @click="$emit('select-conversation', conversation.id)">
        <span>{{ conversation.title }}</span>
        <button @click.stop="conversationsStore.deleteConversation(conversation.id)" class="text-gray-400 hover:text-white">
          <Trash2 class="w-4 h-4" />
        </button>
      </a>
    </nav>
    <div class="p-4 border-t border-gray-700 h-20 flex items-center">
      <a href="#" @click="$emit('show-settings')" class="flex items-center py-2 px-4 rounded hover:bg-gray-700">
        <Settings class="w-5 h-5 mr-3" />
        <span v-if="isExpanded">Settings</span>
      </a>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { Plus, Settings, Menu, Trash2 } from 'lucide-vue-next';
import { useConversationsStore } from '../stores/conversations';

const conversationsStore = useConversationsStore();

defineProps({
  isExpanded: {
    type: Boolean,
    default: true,
  },
});

defineEmits(['toggle-sidebar', 'show-settings', 'new-chat', 'select-conversation']);
</script>

<style scoped>
/* Scoped styles for sidebar */
</style>

