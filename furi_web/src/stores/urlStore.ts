import { defineStore } from 'pinia';
import { ref } from 'vue';

export type ShortenedUrl = {
  short_url: string;
  original_url: string;
};

export const useUrlStore = defineStore('urls', () => {
  const shortenedUrls = ref<ShortenedUrl[]>([]);

  const settedArray = (urls: ShortenedUrl[]): ShortenedUrl[] => {
    const uniqueIdsSet = new Set();

    return urls.filter((obj) => {
      if (uniqueIdsSet.has(obj.short_url)) return false;

      uniqueIdsSet.add(obj.short_url);
      return true;
    });
  };

  const getKnownUrls = (): ShortenedUrl[] => {
    if (!shortenedUrls.value) {
      const fromStorage = localStorage.getItem('known_urls');

      if (!fromStorage) return [];

      const parsed = (JSON.parse(fromStorage) as ShortenedUrl[]) ?? [];

      shortenedUrls.value = settedArray(parsed);

      localStorage.setItem('known_urls', JSON.stringify(shortenedUrls.value));

      return shortenedUrls.value;
    }

    return shortenedUrls.value;
  };

  const deleteUrl = (short_url: string) => {
    shortenedUrls.value = getKnownUrls();

    const hasIndex = shortenedUrls.value.findIndex((r) => r.short_url === short_url);

    if (hasIndex !== -1) shortenedUrls.value.splice(hasIndex, 1);

    localStorage.setItem('known_urls', JSON.stringify(shortenedUrls.value));
  };

  const addUrl = (url: ShortenedUrl) => {
    shortenedUrls.value = getKnownUrls();

    shortenedUrls.value.unshift({ ...url });

    shortenedUrls.value = settedArray(shortenedUrls.value);

    localStorage.setItem('known_urls', JSON.stringify(shortenedUrls.value));
  };

  return {
    shortenedUrls,
    addUrl,
    getKnownUrls,
    deleteUrl
  };
});
