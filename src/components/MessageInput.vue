<template>
  <div class="p-4 h-auto">
    <div v-if="imageBase64" class="relative w-24 h-24 mb-2">
      <img :src="imageBase64" alt="Image preview" class="w-full h-full object-cover rounded-lg" />
      <button @click="removeImage" class="absolute top-0 right-0 bg-gray-800 rounded-full p-1 text-white">
        <X class="w-4 h-4" />
      </button>
    </div>
    <div class="relative">
      <textarea
        v-model="newMessage"
        @keydown.enter.prevent="handleEnter"
        placeholder="Type your message..."
        rows="1"
        class="w-full px-12 py-2 pr-12 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white resize-none overflow-hidden"
        style="min-height: 48px; max-height: 200px;"
      ></textarea>
      <input type="file" ref="fileInput" @change="handleFileChange" accept="image/*" class="hidden" />
      <button @click="triggerFileInput" class="absolute inset-y-0 left-0 flex items-center px-4 text-gray-400 hover:text-white">
        <Paperclip class="w-6 h-6" />
      </button>
      <button @click="sendMessage" class="absolute inset-y-0 right-0 flex items-center px-4 text-gray-400 hover:text-white">
        <Send class="w-6 h-6" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import { Send, Paperclip, X } from 'lucide-vue-next';

const newMessage = ref('');
const imageBase64 = ref<string | null>(null);
const fileInput = ref<HTMLInputElement | null>(null);

const emit = defineEmits(['send-message']);

const sendMessage = () => {
  if (newMessage.value.trim() === '' && !imageBase64.value) return;
  
  emit('send-message', { 
    text: newMessage.value,
    image: imageBase64.value 
  });

  newMessage.value = '';
  imageBase64.value = null;
  if (fileInput.value) {
    fileInput.value.value = '';
  }
};

const triggerFileInput = () => {
  fileInput.value?.click();
};

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    const reader = new FileReader();
    reader.onload = (e) => {
      imageBase64.value = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }
};

const removeImage = () => {
  imageBase64.value = null;
  if (fileInput.value) {
    fileInput.value.value = '';
  }
};

const handleEnter = (event: KeyboardEvent) => {
  if (event.shiftKey) {
    newMessage.value += '\n';
  } else {
    sendMessage();
  }
};

watch(newMessage, () => {
  nextTick(() => {
    const textarea = document.querySelector('textarea');
    if (textarea) {
      textarea.style.height = 'auto';
      textarea.style.height = textarea.scrollHeight + 'px';
    }
  });
});
</script>
