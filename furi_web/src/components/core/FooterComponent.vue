<script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import GithubIcon from '../icons/GithubIcon.vue';
  import LinkedinIcon from '../icons/LinkedinIcon.vue';

  type ThemeMode = 'system' | 'light' | 'dark';

  const theme = ref<ThemeMode>('system');

  const applyTheme = (mode: ThemeMode) => {
    const html = document.documentElement;
    if (mode === 'dark') {
      html.classList.add('dark');
    } else if (mode === 'light') {
      html.classList.remove('dark');
    } else {
      // system: follow OS preference
      if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        html.classList.add('dark');
      } else {
        html.classList.remove('dark');
      }
    }
  };

  const toggleTheme = () => {
    const order: ThemeMode[] = ['system', 'light', 'dark'];
    const idx = order.indexOf(theme.value);
    theme.value = order[(idx + 1) % order.length];
    localStorage.setItem('theme', theme.value);
    applyTheme(theme.value);
  };

  onMounted(() => {
    const stored = localStorage.getItem('theme') as ThemeMode | null;
    if (stored && ['system', 'light', 'dark'].includes(stored)) {
      theme.value = stored;
    }
    applyTheme(theme.value);
  });
</script>

<template>
  <footer class="w-full max-w-md">
    <div class="flex justify-center mt-8 space-x-5">
      <button
        @click="toggleTheme"
        class="text-gray-500 hover:text-gray-900 dark:hover:text-gray-200 transition-colors"
        :title="`Theme: ${theme}`"
      >
        <!-- Sun icon (shown in dark mode) -->
        <svg v-if="theme === 'dark'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v1m0 16v1m8.66-13.66l-.71.71M4.05 19.95l-.71.71M21 12h-1M4 12H3m16.66 7.66l-.71-.71M4.05 4.05l-.71-.71M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
        <!-- Moon icon (shown in light mode) -->
        <svg v-else-if="theme === 'light'" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M21 12.79A9 9 0 1111.21 3a7 7 0 009.79 9.79z" />
        </svg>
        <!-- Monitor icon (shown in system mode) -->
        <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
      </button>
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
