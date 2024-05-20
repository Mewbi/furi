<script setup lang="ts">
  import { NotificationType, useNotifyStore } from '../../stores/notifyStore';
  import { ShortenedUrl } from '../../stores/urlStore'

  let shortenedes: ShortenedUrl[] = [
    { short_url: 'short.ly/3rFg4', original_url: 'https://github.com/Mewbi' },
    { short_url: 'short.ly/7tYh1', original_url: 'https://github.com/Mewbi' },
  ];

  const notifyStore = useNotifyStore();
  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      notifyStore.notify('Copied to clipboard!', NotificationType.Success);
    } catch (e) {
      notifyStore.notify('There was an error when copying the clipboard', NotificationType.Error);
    }
  };

  const getUrl = (url: string) => {
    copyToClipboard(url);
  };
</script>
<template>
  <h2 class="text-lg text-center font-bold text-gray-900 dark:text-white mb-4">
    Your Shortened URLs
  </h2>
  <div class="space-y-4">
    <div
      v-for="short in shortenedes"
      class="flex items-center justify-between bg-gray-100 dark:bg-gray-700 p-3 rounded-md"
    >
      <div>
        <p class="text-sm text-gray-900 dark:text-white mb-1">{{ short.short_url }}</p>
        <p class="text-xs text-gray-500">{{ short.original_url }}</p>
      </div>
      <button
        class="dark:text-white inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3"
        @click="getUrl(short.short_url)"
      >
        Copy
      </button>
    </div>
  </div>
</template>
