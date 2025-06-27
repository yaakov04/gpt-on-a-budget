<script setup lang="ts">
import Sidebar from './components/Sidebar.vue';
</script>

<template>
  <div class="flex h-screen bg-gray-100">
    <!-- Sidebar -->
    <Sidebar />

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <header class="flex items-center justify-between p-4 bg-white border-b border-gray-200">
        <h1 class="text-xl font-semibold">GPT on a Budget</h1>
        <!-- Future header content like user profile, settings, etc. -->
      </header>

      <main class="flex-1 overflow-x-hidden overflow-y-auto bg-gray-100 p-4">
        <!-- Content will go here (e.g., chat view) -->
        <router-view></router-view>
      </main>
    </div>
  </div>
</template>

<style>
/* Global styles can go here if not handled by Tailwind */
</style>
