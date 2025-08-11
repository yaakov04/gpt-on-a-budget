<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import Message from './Message.vue';
import MessageInput from './MessageInput.vue';
import { useConversationsStore } from '../stores/conversations';
import { invoke } from '@tauri-apps/api/core';

const conversationsStore = useConversationsStore();
const isLoading = ref(false);
const messagesContainer = ref<HTMLElement | null>(null);

const scrollToBottom = () => {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
};

watch(() => conversationsStore.activeConversation?.messages, () => {
  scrollToBottom();
}, { deep: true });

const handleSendMessage = async (text: string) => {
  if (!conversationsStore.activeConversation) return;

  const conversationId = conversationsStore.activeConversation.id;
  
  // Add user message and persist it
  await conversationsStore.addMessageToConversation(conversationId, 'user', text);

  isLoading.value = true;

  try {
    // Prepare messages for the LLM API call
    const apiMessages = conversationsStore.activeConversation.messages.map(msg => ({
      role: msg.role,
      content: msg.content,
    }));

    const response: { role: string; content: string } = await invoke('chat_with_llm', {
      messages: apiMessages,
    });

    // Add assistant response and persist it
    await conversationsStore.addMessageToConversation(conversationId, 'assistant', response.content);

  } catch (error) {
    console.error('Error calling chat_with_llm:', error);
    // Optionally add an error message to the conversation
    await conversationsStore.addMessageToConversation(conversationId, 'assistant', `Error: ${error}`);
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <div v-if="conversationsStore.activeConversation" class="flex flex-col h-full">
    <!-- Messages Display -->
    <div ref="messagesContainer" class="flex-1 overflow-y-auto p-4 space-y-4">
      <Message 
        v-for="message in conversationsStore.activeConversation?.messages" 
        :key="message.id" 
        :content="message.content" 
        :role="message.role" 
      />
      <div v-if="isLoading" class="flex justify-center items-center">
        <div class="loader"></div> <!-- Basic loader -->
      </div>
    </div>

    <!-- Message Input -->
    <MessageInput @send-message="handleSendMessage" :is-loading="isLoading" />
  </div>
  <div v-else class="flex flex-col h-full items-center justify-center">
    <h2 class="text-2xl font-bold mb-4">No chat selected</h2>
    <p class="text-gray-400 mb-6">Create a new chat to start messaging.</p>
    <button @click="conversationsStore.addConversation()" class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
      New Chat
    </button>
  </div>
</template>

<style scoped>
.loader {
  border: 4px solid #f3f3f3; /* Light grey */
  border-top: 4px solid #3498db; /* Blue */
  border-radius: 50%;
  width: 24px;
  height: 24px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>