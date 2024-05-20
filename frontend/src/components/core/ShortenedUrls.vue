<script setup lang="ts">
  import { NotificationType, useNotifyStore } from '../../stores/notifyStore';
  import { useUrlStore } from '../../stores/urlStore';
  import CloseIcon from '../icons/CloseIcon.vue';

  const notifyStore = useNotifyStore();
  const urlStore = useUrlStore();

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

  const deleteUrl = (url_short: string) => {
    urlStore.deleteUrl(url_short);
  };

</script>
<template>
  <div v-if="urlStore.shortenedUrls.length" class="w-full max-w-md mt-8 mb-2">
    <h2 class="text-lg text-center font-bold text-gray-900 dark:text-white mb-4">
      Your Shortened URLs
    </h2>
    <div class="space-y-4">
      <div
        v-for="short in urlStore.shortenedUrls"
        class="relative bg-gray-100 dark:bg-gray-700 p-3 rounded-md"
      >
        <div class="w-full flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-900 dark:text-white mb-1">{{ short.short_url }}</p>
            <p class="text-xs text-gray-500">{{ short.original_url }}</p>
          </div>
          <button
            @click="getUrl(short.short_url)"
            class="dark:text-white hover:text-gray-500 inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 rounded-md px-3"
          >
            Copy
          </button>
        </div>
        <button
          @click="deleteUrl(short.short_url)"
          class="absolute -top-2 -left-2" >
          <CloseIcon />
        </button>
      </div>
    </div>
  </div>
</template>
