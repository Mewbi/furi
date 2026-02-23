<script setup lang="ts">
  import { onMounted } from 'vue';
  import { NotificationType, useNotifyStore } from '../../stores/notifyStore';
  import { useUrlStore } from '../../stores/urlStore';
  import CloseIcon from '../icons/CloseIcon.vue';
  import CopyIcon from '../icons/CopyIcon.vue';
  import AnalyticsIcon from '../icons/AnalyticsIcon.vue';
  import { RouterLink } from 'vue-router';

  const notifyStore = useNotifyStore();
  const urlStore = useUrlStore();

  onMounted(() => {
    urlStore.getKnownUrls();
  });

  const fallbackCopyTextToClipboard = async (text: string) => {
    const textArea = document.createElement('textarea');
    textArea.value = text;

    // Avoid scrolling to bottom
    textArea.style.top = '0';
    textArea.style.left = '0';
    textArea.style.position = 'fixed';

    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();

    try {
      document.execCommand('copy');
      notifyStore.notify('Copied to clipboard!', NotificationType.Success);
    } catch (err) {
      notifyStore.notify('There was an error when copying the clipboard', NotificationType.Error);
    }
    document.body.removeChild(textArea);
  };

  const copyToClipboard = async (text: string) => {
    if (!navigator.clipboard) {
      fallbackCopyTextToClipboard(text);
      return;
    }
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

  const computedUrl = (url: string)  => {
    const limit = 50;
    if (url.length > limit) {
      return url.substring(0, limit) + "..."
    }
    return url
  }

  const shortCode = (url: string) => url.split('/').pop() ?? url;

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
            <a
              target="_blank"
              :href="`${short.short_url}`"
              class="hover:underline hover:text-gray-900 me-4 md:me-6 flex gap-2"
            >
              <p class="text-sm text-gray-900 dark:text-white mb-1">{{ short.short_url }}</p>
            </a>
            <p class="text-xs text-gray-500 text-wrap">{{ computedUrl(short.original_url) }}</p>
          </div>
          <div class="flex gap-1 items-center">
            <button @click="getUrl(short.short_url)" title="Copy"
              class="inline-flex items-center justify-center rounded-md p-1.5 transition-colors hover:bg-gray-200 dark:hover:bg-gray-600">
              <CopyIcon />
            </button>
            <RouterLink :to="{ name: 'analytics', params: { id: shortCode(short.short_url) } }" title="Analytics"
              class="inline-flex items-center justify-center rounded-md p-1.5 transition-colors hover:bg-gray-200 dark:hover:bg-gray-600">
              <AnalyticsIcon />
            </RouterLink>
          </div>
        </div>
        <button @click="deleteUrl(short.short_url)" class="absolute -top-2 -left-2">
          <CloseIcon />
        </button>
      </div>
    </div>
  </div>
</template>
