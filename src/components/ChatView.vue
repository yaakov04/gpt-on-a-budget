<template>
  <div class="flex flex-col h-full">
    <!-- Messages Display -->
    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      <Message v-for="message in messages" :key="message.id" :content="message.text" :is-user="message.isUser" />
    </div>

    <!-- Message Input -->
    <MessageInput @send-message="handleSendMessage" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Message from './Message.vue';
import MessageInput from './MessageInput.vue';

const messages = ref([
  { id: 1, text: 'Hello!', isUser: false },
  { id: 2, text: 'Hi there! How can I help you today?', isUser: true },
  { id: 3, text: 'Here is some `code`', isUser: false },
]);

const handleSendMessage = (text: string) => {
  messages.value.push({
    id: Date.now(),
    text,
    isUser: true,
  });

  // Simulate a bot response
  setTimeout(() => {
    messages.value.push({
      id: Date.now() + 1,
      text: 'This is a **simulated** response with [markdown](https://www.markdownguide.org/).',
      isUser: false,
    });
  }, 1000);
};
</script>