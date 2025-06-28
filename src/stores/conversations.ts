import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface Message {
  id: number;
  text: string;
  isUser: boolean;
}

export interface Conversation {
  id: number;
  title: string;
  messages: Message[];
}

export const useConversationsStore = defineStore('conversations', () => {
  const conversations = ref<Conversation[]>([
    { id: 1, title: 'New Chat 1', messages: [{ id: 1, text: 'Hello!', isUser: false }] },
    { id: 2, title: 'New Chat 2', messages: [{ id: 1, text: 'Hi there!', isUser: false }] },
  ]);
  const activeConversationId = ref<number | null>(conversations.value.length > 0 ? conversations.value[0].id : null);

  const activeConversation = computed(() => {
    return conversations.value.find(conv => conv.id === activeConversationId.value);
  });

  function addConversation() {
    const newId = Date.now();
    conversations.value.push({
      id: newId,
      title: `New Chat ${conversations.value.length + 1}`,
      messages: [],
    });
    activeConversationId.value = newId;
  }

  function deleteConversation(id: number) {
    const index = conversations.value.findIndex(conv => conv.id === id);
    if (index !== -1) {
      conversations.value.splice(index, 1);
      if (activeConversationId.value === id) {
        activeConversationId.value = conversations.value.length > 0 ? conversations.value[0].id : null;
      }
    }
  }

  function selectConversation(id: number) {
    activeConversationId.value = id;
  }

  function addMessageToConversation(conversationId: number, message: Message) {
    const conversation = conversations.value.find(conv => conv.id === conversationId);
    if (conversation) {
      conversation.messages.push(message);
    }
  }

  return {
    conversations,
    activeConversationId,
    activeConversation,
    addConversation,
    deleteConversation,
    selectConversation,
    addMessageToConversation,
  };
});
