<script setup lang="ts">
  import { onMounted } from 'vue';
  import NotifyComponent from './components/core/NotifyComponent.vue';
  import { RouterView } from 'vue-router';

  onMounted(() => {
    const stored = localStorage.getItem('theme');
    if (stored === 'dark') {
      document.documentElement.classList.add('dark');
    } else if (stored === 'light') {
      document.documentElement.classList.remove('dark');
    } else {
      // system preference
      if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.documentElement.classList.add('dark');
      }
    }
  });
</script>

<template>
  <RouterView />
  <NotifyComponent class="fixed bottom-0 left-0 z-50 p-4" />
</template>
