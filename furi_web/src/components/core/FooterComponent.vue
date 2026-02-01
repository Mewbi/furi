<script setup lang="ts">
  import GithubIcon from '../icons/GithubIcon.vue';
  import LinkedinIcon from '../icons/LinkedinIcon.vue';
</script>

<template>
  <footer class="w-full max-w-md">
    <div class="flex justify-center mt-8 space-x-5">
      <a
        target="_blank"
        href="https://github.com/Mewbi/furi"
        class="hover:underline hover:text-gray-900 me-4 md:me-6 flex gap-2"
      >
        <GithubIcon />
      </a>
      <a
        target="_blank"
        href="https://www.linkedin.com/in/felipe-fernandes-gsc/"
        class="hover:underline hover:text-gray-900 me-4 md:me-6 flex gap-2"
      >
        <LinkedinIcon />
      </a>
    </div>

    <div>
      <hr class="my-2 border-gray-400 sm:mx-auto lg:my-2" />
      <span class="block text-sm text-gray-500 sm:text-center">
        © {{ new Date().getFullYear() }}
        <RouterLink :to="{ name: 'home' }" class="hover:underline"> - Furi</RouterLink>
      </span>
    </div>
  </footer>
</template>

<style scoped></style>
