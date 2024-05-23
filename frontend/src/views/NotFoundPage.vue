<script setup lang="ts">
  import FooterComponent from '../components/core/FooterComponent.vue';
  import { RouterLink } from 'vue-router';
</script>
<template>
  <div
    class="flex flex-col justify-between w-full h-full min-h-screen bg-white dark:bg-gray-800 p-8"
  >
    <div></div>

    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full">
      <div class="w-full max-w-lg flex flex-col items-center">
        <RouterLink :to="{ name: 'home' }">
          <picture>
            <source srcset="/not_found_light.png" media="(prefers-color-scheme: dark)" />
            <img src="/not_found_dark.png" class="w-60 h-60 mb-6" />
          </picture>
        </RouterLink>

        <div class="text-center dark:text-white w-full max-w-md text-wrap space-y-2 pl-1 pr-1">
          <h1 class="text-2xl font-bold mb-6">Oops. Page not found!</h1>
          <p>You may accessed an invalid URL.</p>
          <p>Check your link again or create a new one <RouterLink :to="{ name: 'home' }"><b>here</b></RouterLink>.</p>
        </div>
      </div>

    </div>

    <!-- Footer section -->
    <div class="flex flex-col items-center text-center">
      <FooterComponent />
    </div>
  </div>
</template>

<style scoped></style>
