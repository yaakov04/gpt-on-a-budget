<template>
  <div class="p-4  h-20">
    <div class="relative">
      <input
        type="text"
        v-model="newMessage"
        @keyup.enter="sendMessage"
        placeholder="Type your message..."
        class="w-full px-4 py-2 pr-12 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
      />
      <button @click="sendMessage" class="absolute inset-y-0 right-0 flex items-center px-4 text-gray-400 hover:text-white">
        <Send class="w-6 h-6" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Send } from 'lucide-vue-next';

const newMessage = ref('');
const emit = defineEmits(['send-message']);

const sendMessage = () => {
  if (newMessage.value.trim() === '') return;
  emit('send-message', newMessage.value);
  newMessage.value = '';
};
</script>
