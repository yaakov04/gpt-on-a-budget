<template>
  <div class="flex flex-col h-full">
    <!-- Messages Display -->
    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      <Message v-for="message in conversationsStore.activeConversation?.messages" :key="message.id" :content="message.text" :is-user="message.isUser" />
    </div>

    <!-- Message Input -->
    <MessageInput @send-message="handleSendMessage" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Message from './Message.vue';
import MessageInput from './MessageInput.vue';
import { useConversationsStore } from '../stores/conversations';
import { invoke } from '@tauri-apps/api/core';

const conversationsStore = useConversationsStore();

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

  try {
    const response: { role: string; content: string } = await invoke('chat_with_llm', {
      messages: conversationsStore.activeConversation?.messages.map(msg => ({ role: msg.isUser ? 'user' : 'assistant', content: msg.text })) || [],
    });

    conversationsStore.addMessageToConversation(conversationsStore.activeConversationId, {
      id: Date.now() + 1,
      text: response.content,
      isUser: false,
    });
  } catch (error) {
    console.error('Error calling chat_with_llm:', error);
    conversationsStore.addMessageToConversation(conversationsStore.activeConversationId, {
      id: Date.now() + 1,
      text: `Error: ${error}`,
      isUser: false,
    });
  } finally {
    isLoading.value = false;
  }
};
</script>