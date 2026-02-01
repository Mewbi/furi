<script setup lang="ts">
  import NotifyComponent from './components/core/NotifyComponent.vue';
  import { RouterView } from 'vue-router';
</script>

<template>
  <RouterView />
  <NotifyComponent class="fixed bottom-0 left-0 z-50 p-4" />
</template>
