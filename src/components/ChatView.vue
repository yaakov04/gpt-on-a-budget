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
import { ref, watch } from 'vue';
import Message from './Message.vue';
import MessageInput from './MessageInput.vue';
import { useConversationsStore } from '../stores/conversations';

const conversationsStore = useConversationsStore();

const messages = ref(conversationsStore.activeConversation?.messages || []);

watch(() => conversationsStore.activeConversation, (newConversation) => {
  messages.value = newConversation?.messages || [];
}, { immediate: true });

const isLoading = ref(false);

const handleSendMessage = async (text: string) => {
  if (!conversationsStore.activeConversationId) return;

  const newMessage = {
    id: Date.now(),
    text,
    isUser: true,
  };
  conversationsStore.addMessageToConversation(conversationsStore.activeConversationId, newMessage);

  isLoading.value = true;

  // Simulate a bot response
  setTimeout(() => {
    if (conversationsStore.activeConversationId) {
      conversationsStore.addMessageToConversation(conversationsStore.activeConversationId, {
        id: Date.now() + 1,
        text: 'This is a **simulated** response with [markdown](https://www.markdownguide.org/).',
        isUser: false,
      });
    }
    isLoading.value = false;
  }, 1000);
};
</script>