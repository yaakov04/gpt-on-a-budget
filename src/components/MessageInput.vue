<template>
  <div class="p-4  h-20">
    <div class="relative">
      <textarea
        v-model="newMessage"
        @keydown.enter.prevent="handleEnter"
        placeholder="Type your message..."
        rows="1"
        class="w-full px-4 py-2 pr-12 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white resize-none overflow-hidden"
        style="min-height: 48px; max-height: 200px;"
      ></textarea>
      <button @click="sendMessage" class="absolute inset-y-0 right-0 flex items-center px-4 text-gray-400 hover:text-white">
        <Send class="w-6 h-6" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import { Send } from 'lucide-vue-next';

const newMessage = ref('');
const emit = defineEmits(['send-message']);

const sendMessage = () => {
  if (newMessage.value.trim() === '') return;
  emit('send-message', newMessage.value);
  newMessage.value = '';
};

const handleEnter = (event: KeyboardEvent) => {
  if (event.shiftKey) {
    newMessage.value += '\n';
  } else {
    sendMessage();
  }
};

watch(newMessage, () => {
  nextTick(() => {
    const textarea = document.querySelector('textarea');
    if (textarea) {
      textarea.style.height = 'auto';
      textarea.style.height = textarea.scrollHeight + 'px';
    }
  });
});
</script>
