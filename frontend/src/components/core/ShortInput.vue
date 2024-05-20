<script setup lang="ts">
  import { ref } from 'vue';
  import WorldIcon from '../icons/WorldIcon.vue';
  import api from '../../services/api';
  import { NotificationType, useNotifyStore } from '../../stores/notifyStore';

  const notifyStore = useNotifyStore();

  // const props = defineProps<{
  //   url: string
  // }>();
  const url = ref('');
  const shortUrl = async() => {
    console.log(url)
    
    const res = await api.shortUrl("minha url xD");
    if (res.error)
      return notifyStore.notify('An error occured to create short url', NotificationType.Error);

    notifyStore.notify('Success creating short url', NotificationType.Success);
  };

</script>
<template>
  <div class="mt-12 w-full max-w-md relative mb-4">
    <div class="absolute inset-y-0 start-0 flex items-center ps-2.5 pointer-events-none">
      <WorldIcon />
    </div>
    <input
      type="text"
      id="url"
      class="bg-gray-100 text-gray-900 text-sm rounded-md block w-full ps-10 p-2.5 dark:bg-gray-700 dark:placeholder-gray-400 dark:text-white "
      placeholder="https://github.com/Mewbi"
    />
    <button
      @click="shortUrl"
      :disabled="
        false
      "
      class="absolute top-0 end-0 p-2.5 h-full text-sm font-medium text-white dark:text-black bg-gray-600 rounded-e-md border hover:bg-gray-700 focus:ring-4 focus:outline-none focus:ring-gray-300 dark:bg-gray-200 dark:hover:bg-gray-300 dark:focus:ring-gray-800"
    >
      Short
    </button>
  </div>
</template>
