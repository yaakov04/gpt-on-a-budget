
<script setup lang="ts">
import { ref } from 'vue';
import { useSettingsStore } from '../stores/settings';

const settingsStore = useSettingsStore();
const apiKeyInput = ref('');

const handleSave = () => {
  settingsStore.saveApiKey(apiKeyInput.value);
};
</script>

<template>
  <div class="fixed inset-0 bg-gray-900 bg-opacity-80 flex items-center justify-center z-50">
    <div class="bg-gray-800 p-8 rounded-lg shadow-xl w-full max-w-md">
      <h2 class="text-2xl font-bold text-white mb-4">Enter API Key</h2>
      <p class="text-gray-400 mb-6">Please enter your OpenAI API key to continue. Your key is stored securely and locally on your device.</p>
      <div class="mb-4">
        <input 
          v-model="apiKeyInput" 
          type="password" 
          placeholder="sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
          class="w-full px-4 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>
      <button 
        @click="handleSave" 
        :disabled="settingsStore.isLoading || !apiKeyInput"
        class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:bg-gray-600 disabled:cursor-not-allowed flex items-center justify-center"
      >
        <span v-if="settingsStore.isLoading" class="loader mr-2"></span>
        Save and Continue
      </button>
    </div>
  </div>
</template>

<style scoped>
.loader {
  border: 2px solid #f3f3f3;
  border-top: 2px solid #3498db;
  border-radius: 50%;
  width: 16px;
  height: 16px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>
