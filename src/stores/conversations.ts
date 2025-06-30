import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// Matching the Rust structs
export interface Message {
  id: number;
  conversation_id: number;
  role: 'user' | 'assistant' | 'system';
  content: string;
  created_at: string;
}

export interface Conversation {
  id: number;
  title: string;
  created_at: string;
  llm_provider: string;
  messages: Message[];
}

export const useConversationsStore = defineStore('conversations', () => {
  const conversations = ref<Conversation[]>([]);
  const activeConversationId = ref<number | null>(null);

  const activeConversation = computed(() => {
    return conversations.value.find(conv => conv.id === activeConversationId.value) || null;
  });

  async function fetchConversations() {
    try {
      const fetchedConversations = await invoke<Conversation[]>('get_conversations');
      conversations.value = fetchedConversations;
      if (conversations.value.length > 0 && !activeConversationId.value) {
        activeConversationId.value = conversations.value[0].id;
      }
    } catch (error) {
      console.error("Failed to fetch conversations:", error);
    }
  }

  async function addConversation() {
    try {
      let newTitle = 'New Chat';
      let counter = 0;
      while (conversations.value.some(conv => conv.title === newTitle + (counter > 0 ? ` ${counter}` : ''))) {
        counter++;
      }
      if (counter > 0) {
        newTitle += ` ${counter}`;
      }
      const newConversation = await invoke<Conversation>('create_conversation', {
        title: newTitle,
      });
      conversations.value.unshift(newConversation); // Add to the top
      activeConversationId.value = newConversation.id;
    } catch (error) {
      console.error("Failed to create conversation:", error);
    }
  }

  async function deleteConversation(id: number) {
    try {
      await invoke('delete_conversation', { id });
      const index = conversations.value.findIndex(conv => conv.id === id);
      if (index !== -1) {
        conversations.value.splice(index, 1);
        if (activeConversationId.value === id) {
          activeConversationId.value = conversations.value.length > 0 ? conversations.value[0].id : null;
        }
      }
    } catch (error) {
      console.error("Failed to delete conversation:", error);
    }
  }

  function selectConversation(id: number) {
    activeConversationId.value = id;
  }

  async function addMessageToConversation(conversationId: number, role: 'user' | 'assistant', content: string) {
    const conversation = conversations.value.find(conv => conv.id === conversationId);
    if (conversation) {
        try {
            const newMessage = await invoke<Message>('add_message', {
                conversationId,
                role,
                content,
            });
            conversation.messages.push(newMessage);
        } catch (error) {
            console.error("Failed to add message:", error);
        }
    }
  }

  return {
    conversations,
    activeConversationId,
    activeConversation,
    fetchConversations,
    addConversation,
    deleteConversation,
    selectConversation,
    addMessageToConversation,
  };
});