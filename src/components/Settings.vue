<template>
  <div class="p-4">
    <h2 class="text-lg font-bold mb-4">Settings</h2>
    <div class="mb-4">
      <label for="apiKey" class="block mb-2">OpenAI API Key</label>
      <input
        id="apiKey"
        v-model="apiKey"
        type="password"
        class="w-full p-2 border rounded"
        placeholder="Enter your OpenAI API Key"
      />
    </div>
    <button @click="saveApiKey" class="bg-blue-500 text-white px-4 py-2 rounded">
      Save
    </button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const apiKey = ref('');

onMounted(async () => {
  try {
    apiKey.value = await invoke('get_api_key');
  } catch (e) {
    console.error("Failed to get API key:", e);
    apiKey.value = '';
  }
});

const saveApiKey = async () => {
  try {
    await invoke('set_api_key', { apiKey: apiKey.value });
    alert('API Key saved!');
  } catch (e) {
    console.error("Failed to save API key:", e);
    alert(`Failed to save API key: ${e}`);
  }
};
</script>
