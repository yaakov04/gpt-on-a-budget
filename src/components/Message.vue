<script setup lang="ts">
import { computed, PropType } from 'vue';
import MarkdownIt from 'markdown-it';
import { MessageContent, ContentBlock } from '../stores/conversations';

const props = defineProps({
  content: {
    type: [String, Array] as PropType<MessageContent>,
    required: true,
  },
  role: {
    type: String as PropType<'user' | 'assistant' | 'system'>,
    required: true,
  },
});

const isUser = computed(() => props.role === 'user');

const md = new MarkdownIt({
  breaks: true,
});

const isArrayContent = computed(() => Array.isArray(props.content));

const renderedMarkdown = (text: string) => md.render(text);

</script>

<template>
  <div class="flex" :class="isUser ? 'justify-end' : 'justify-start'">
    <div 
      class="prose max-w-lg px-4 py-2 rounded-lg prose-sm dark:prose-invert prose-p:my-2 prose-pre:my-2 prose-pre:bg-gray-900 prose-pre:p-2 prose-pre:rounded"
      :class="isUser ? 'bg-blue-800 text-white' : 'bg-gray-700 text-gray-200'"
    >
      <!-- If content is an array of blocks -->
      <div v-if="isArrayContent">
        <div v-for="(block, index) in (content as ContentBlock[])" :key="index">
          <div v-if="block.type === 'text' && block.text" v-html="renderedMarkdown(block.text)"></div>
          <div v-if="block.type === 'image_url' && block.image_url" class="my-2">
            <img :src="block.image_url.url" alt="User image" class="max-w-xs rounded-lg" />
          </div>
        </div>
      </div>
      <!-- If content is a simple string (legacy or assistant message) -->
      <div v-else v-html="renderedMarkdown(content as string)"></div>
    </div>
  </div>
</template>

<style scoped>
/* Add any additional scoped styles if needed */
</style>