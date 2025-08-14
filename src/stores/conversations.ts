import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// --- Multimodal Types ---
export interface ImageUrl {
  url: string;
}

export interface ContentBlock {
  type: 'text' | 'image_url';
  text?: string;
  image_url?: ImageUrl;
}

export type MessageContent = string | ContentBlock[];

// --- Store Types ---
// Interface for messages received from the DB (content is a JSON string)
interface RawMessage {
  id: number;
  conversation_id: number;
  role: 'user' | 'assistant' | 'system';
  content: string; // This will be a JSON string
  created_at: string;
}

// Interface for messages used in the UI (content is a parsed object)
export interface Message {
  id: number;
  conversation_id: number;
  role: 'user' | 'assistant' | 'system';
  content: MessageContent;
  created_at: string;
}

export interface Conversation {
  id: number;
  title: string;
  created_at: string;
  llm_provider: string;
  messages: Message[];
}

// Helper to parse message content
function parseMessageContent(messages: RawMessage[]): Message[] {
  return messages.map(msg => {
    try {
      // Assistant messages content are plain text, not JSON
      if (msg.role === 'assistant') {
        return { ...msg, content: msg.content };
      }
      return { ...msg, content: JSON.parse(msg.content) };
    } catch (e) {
      // If parsing fails, treat it as a plain string (for legacy messages)
      return { ...msg, content: msg.content };
    }
  });
}

export const useConversationsStore = defineStore('conversations', () => {
  const conversations = ref<Conversation[]>([]);
  const activeConversationId = ref<number | null>(null);

  const activeConversation = computed(() => {
    return conversations.value.find(conv => conv.id === activeConversationId.value) || null;
  });

  async function fetchConversations() {
    try {
      const fetchedConversations = await invoke<any[]>('get_conversations');
      conversations.value = fetchedConversations.map(conv => ({
        ...conv,
        messages: parseMessageContent(conv.messages),
      }));

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
        llmProvider: 'openai',
      });
      conversations.value.unshift(newConversation);
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

  async function addMessageToConversation(conversationId: number, role: 'user' | 'assistant', content: MessageContent) {
    const conversation = conversations.value.find(conv => conv.id === conversationId);
    if (conversation) {
        try {
            // Stringify the content object for DB storage
            const contentForDb = JSON.stringify(content);

            const newMessage: RawMessage = await invoke('add_message', {
                conversationId,
                role,
                content: contentForDb,
            });

            // Push the parsed message to the store
            conversation.messages.push({
              ...newMessage,
              content: content, // Use the rich content object we already have
            });
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
