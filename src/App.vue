<script setup lang="ts">
import Sidebar from './components/Sidebar.vue';
import ChatView from './components/ChatView.vue';
import Dropdown from './components/Dropdown.vue';
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { ChevronDown, User, LogOut } from 'lucide-vue-next';

const models = ref(['GPT-4', 'GPT-3.5-Turbo', 'Gemini-Pro']);
const selectedModel = ref('GPT-4');

const isSidebarExpanded = ref(true);

const toggleSidebar = () => {
  isSidebarExpanded.value = !isSidebarExpanded.value;
};

const handleResize = () => {
  if (window.innerWidth < 768) {
    isSidebarExpanded.value = false;
  } else {
    isSidebarExpanded.value = true;
  }
};

onMounted(() => {
  window.addEventListener('resize', handleResize);
  handleResize(); // Initial check
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
});

</script>

<template>
  <div class="flex h-screen bg-gray-800 text-white">
    <!-- Sidebar -->
    <Sidebar :is-expanded="isSidebarExpanded" @toggle-sidebar="toggleSidebar" />

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden transition-all duration-300 ease-in-out" >
      <header class="flex items-center justify-between p-4 bg-gray-800 border-b border-gray-700 h-20">
        <!-- Model Selector -->
        <Dropdown>
          <template #trigger>
            <span class="font-semibold">{{ selectedModel }}</span>
            <ChevronDown class="w-5 h-5 ml-1" />
          </template>
          <template #content>
            <a v-for="model in models" :key="model" href="#" @click="selectedModel = model"
              class="block px-4 py-2 text-sm text-gray-300 hover:bg-gray-700">
              {{ model }}
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
        <ChatView />
      </main>
    </div>
  </div>
</template>

<style>
/* Global styles can go here if not handled by Tailwind */
</style>
