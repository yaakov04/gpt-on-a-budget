
import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export const useSettingsStore = defineStore('settings', () => {
  const isApiKeySet = ref(false);
  const isLoading = ref(true);

  async function checkApiKey() {
    isLoading.value = true;
    try {
      await invoke('get_api_key');
      isApiKeySet.value = true;
    } catch (error) {
      console.error('API key not found or invalid:', error);
      isApiKeySet.value = false;
    } finally {
      isLoading.value = false;
    }
  }

  async function saveApiKey(apiKey: string) {
    if (!apiKey) return;
    isLoading.value = true;
    try {
      await invoke('set_api_key', { apiKey });
      isApiKeySet.value = true;
    } catch (error) {
      console.error('Failed to save API key:', error);
      isApiKeySet.value = false;
    } finally {
      isLoading.value = false;
    }
  }

  return {
    isApiKeySet,
    isLoading,
    checkApiKey,
    saveApiKey,
  };
});
