<script setup lang="ts">
import Sidebar from './components/Sidebar.vue';
import ChatView from './components/ChatView.vue';
import Settings from './components/Settings.vue';
import Dropdown from './components/Dropdown.vue';
import ApiKeyModal from './components/ApiKeyModal.vue';
import { ref, onMounted, onUnmounted } from 'vue';
import { ChevronDown, User, LogOut } from 'lucide-vue-next';
import { useConversationsStore } from './stores/conversations';
import { useSettingsStore } from './stores/settings';
import { invoke } from '@tauri-apps/api/core';

// Interface for the model object from the backend
interface LlmModel {
  id: number;
  model_name: string;
  provider: string;
  is_default: boolean;
}

const models = ref<LlmModel[]>([]);
const selectedModel = ref<LlmModel | null>(null);

const isSidebarExpanded = ref(true);
const showSettings = ref(false);
const conversationsStore = useConversationsStore();
const settingsStore = useSettingsStore();

const toggleSidebar = () => {
  isSidebarExpanded.value = !isSidebarExpanded.value;
};

const handleNewChat = () => {
  conversationsStore.addConversation();
  showSettings.value = false;
};

const handleSelectConversation = (id: number) => {
  conversationsStore.selectConversation(id);
  showSettings.value = false;
};

const handleResize = () => {
  if (window.innerWidth < 768) {
    isSidebarExpanded.value = false;
  } else {
    isSidebarExpanded.value = true;
  }
};

const loadModels = async () => {
  try {
    models.value = await invoke('get_available_models');
    selectedModel.value = await invoke('get_default_model');
  } catch (error) {
    console.error("Failed to load LLM models:", error);
    // Optionally, set some default/error state
  }
};

const handleModelSelect = async (model: LlmModel) => {
  selectedModel.value = model;
  try {
    await invoke('set_default_model', { modelId: model.id });
  } catch (error) {
    console.error("Failed to set default model:", error);
    // Optionally, revert the selection or show an error to the user
  }
};

onMounted(async () => {
  window.addEventListener('resize', handleResize);
  handleResize(); // Initial check
  await settingsStore.checkApiKey();
  if (settingsStore.isApiKeySet) {
    conversationsStore.fetchConversations(); // Fetch conversations on startup
  }
  await loadModels();
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
});

</script>

<template>
  <div class="flex h-screen bg-gray-800 text-white">
    <ApiKeyModal v-if="!settingsStore.isApiKeySet" />
    <!-- Sidebar -->
    <Sidebar
      :is-expanded="isSidebarExpanded"
      @toggle-sidebar="toggleSidebar"
      @show-settings="showSettings = true"
      @new-chat="handleNewChat"
      @select-conversation="handleSelectConversation"
    />

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden transition-all duration-300 ease-in-out" >
      <header class="flex items-center justify-between p-4 bg-gray-800 border-b border-gray-700 h-20">
        <!-- Model Selector -->
        <Dropdown v-if="selectedModel">
          <template #trigger>
            <span class="font-semibold">{{ selectedModel.model_name }}</span>
            <ChevronDown class="w-5 h-5 ml-1" />
          </template>
          <template #content>
            <a v-for="model in models" :key="model.id" href="#" @click.prevent="handleModelSelect(model)"
              class="block px-4 py-2 text-sm text-gray-300 hover:bg-gray-700">
              {{ model.model_name }}
            </a>
          </template>
        </Dropdown>

        <!-- App Title -->
        <h1 class="text-xl font-bold tracking-wider">GB</h1>

        <!-- User Menu -->
        <Dropdown menu-align="right">
          <template #trigger>
            <div class="w-8 h-8 bg-blue-600 rounded-full flex items-center justify-center">
              <User class="w-5 h-5" />
            </div>
          </template>
          <template #content>
            <a href="#" class="flex items-center px-4 py-2 text-sm text-gray-300 hover:bg-gray-700">
              <LogOut class="w-4 h-4 mr-2" />
              Cerrar Sesión
            </a>
          </template>
        </Dropdown>
      </header>

      <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gray-800">
        <Settings v-if="showSettings" />
        <ChatView v-else />
      </main>
    </div>
  </div>
</template>

<style>
/* Global styles can go here if not handled by Tailwind */
</style>

