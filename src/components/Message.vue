'''<script setup lang="ts">
import { computed } from 'vue';
import MarkdownIt from 'markdown-it';

const props = defineProps<{ 
  content: string;
  role: 'user' | 'assistant' | 'system';
}>();

const isUser = computed(() => props.role === 'user');

const md = new MarkdownIt();

const renderedContent = computed(() => md.render(props.content));
</script>

<template>
  <div class="flex" :class="isUser ? 'justify-end' : 'justify-start'">
    <div 
      class="max-w-lg px-4 py-2 rounded-lg prose dark:prose-invert prose-p:my-2 prose-pre:my-2 prose-pre:bg-gray-900 prose-pre:p-2 prose-pre:rounded"
      :class="isUser ? 'bg-blue-800 text-white' : 'bg-gray-700 text-gray-200'"
      v-html="renderedContent"
    ></div>
  </div>
</template>

<style scoped>
/* Add any additional scoped styles if needed */
</style>
'''