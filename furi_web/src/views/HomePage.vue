<script setup lang="ts">
  import ShortInput from '../components/core/ShortInput.vue';
  import ShortenedUrls from '../components/core/ShortenedUrls.vue';
  import FooterComponent from '../components/core/FooterComponent.vue';
</script>
<template>
  <div
    class="flex flex-col justify-between w-full h-full min-h-screen bg-white dark:bg-gray-800 p-8"
  >
    <div></div>

    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full">
      <div class="w-full max-w-lg flex flex-col items-center">
        <img src="/logo_dark.png" class="w-52 h-52 mb-6 block dark:hidden" />
        <img src="/logo_light.png" class="w-52 h-52 mb-6 hidden dark:block" />

        <div class="dark:text-white w-full max-w-md text-wrap text-center space-y-2 pl-1 pr-1">
          <p>A <b>F</b>ast way to short your <b>URI</b>.</p>
          <p>It's simple, just paste your URL and get a small one.</p>
          <p>And more, you can check <b>analytics</b> metrics of your URL.</p>
        </div>
      </div>

      <div class="w-full max-w-lg flex flex-col items-center">
        <ShortInput />
        <ShortenedUrls />
      </div>
    </div>

    <!-- Footer section -->
    <div class="flex flex-col items-center text-center">
      <FooterComponent />
    </div>
  </div>
</template>

<style scoped></style>
