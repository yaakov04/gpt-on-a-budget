<template>
  <div class="flex" :class="isUser ? 'justify-end' : 'justify-start'">
    <div class="max-w-lg px-4 py-2 rounded-lg" :class="isUser ? 'bg-blue-800 text-white' : 'bg-gray-700 text-gray-200'" v-html="renderedContent"></div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import MarkdownIt from 'markdown-it';

const props = defineProps<{ 
  content: string;
  isUser: boolean;
}>();

const md = new MarkdownIt();

const renderedContent = computed(() => md.render(props.content));
</script>
