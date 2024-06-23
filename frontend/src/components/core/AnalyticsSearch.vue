<script setup lang="ts">
  import { ref } from 'vue';
  import WorldIcon from '../icons/WorldIcon.vue';
  import api from '../../services/api';
  import { NotificationType, useNotifyStore } from '../../stores/notifyStore';

  const notifyStore = useNotifyStore();

  const url = ref('');
  const fromDate = ref('');
  const toDate = ref('');
  const aggregation = ref('1min');

  const getAnalytics = async () => {
    // Convert date strings to timestamps
    const fromTimestamp = new Date(fromDate.value).getTime() / 1000;
    const toTimestamp = new Date(toDate.value).getTime() / 1000;

    const res = await api.getAnalytics('12jkn3a', fromTimestamp, toTimestamp, '1min');
    if ('error' in res) {
      if ('message' in res && res.message)
        return notifyStore.notify(`${res.message}`, NotificationType.Error);
      return notifyStore.notify('An error occured to get uri metrics', NotificationType.Error);
    }
    console.log(res);
    notifyStore.notify('Success creating short url', NotificationType.Success);
  };
</script>
<template>
  <div class="mt-12 w-full max-w-md relative mb-4">
    <div class="absolute inset-y-0 start-0 flex items-center ps-2.5 pointer-events-none">
      <WorldIcon />
    </div>

    <div class="flex flex-col space-y-4">
      <input
        type="text"
        v-model="url"
        class="bg-gray-100 text-gray-900 text-sm rounded-md block w-full ps-10 p-2.5 dark:bg-gray-700 dark:placeholder-gray-400 dark:text-white"
        placeholder="https://github.com/Mewbi"
      />

      <div class="flex flex-col space-y-2">
        <label for="fromDate">From Date:</label>
        <input
          type="date"
          v-model="fromDate"
          id="fromDate"
          class="bg-gray-100 text-gray-900 text-sm rounded-md block w-full p-2.5 dark:bg-gray-700 dark:placeholder-gray-400 dark:text-white"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <label for="toDate">To Date:</label>
        <input
          type="date"
          v-model="toDate"
          id="toDate"
          class="bg-gray-100 text-gray-900 text-sm rounded-md block w-full p-2.5 dark:bg-gray-700 dark:placeholder-gray-400 dark:text-white"
        />
      </div>

      <div class="flex flex-col space-y-2">
        <label for="aggregation">Aggregation:</label>
        <select
          v-model="aggregation"
          id="aggregation"
          class="block w-full bg-gray-200 border border-gray-200 text-gray-700 py-3 px-4 pr-8 rounded leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
        >
          <option value="1min">1 Minute</option>
          <option value="1hour">1 Hour</option>
          <option value="1day">1 Day</option>
        </select>
      </div>
      
      <button
        @click="getAnalytics"
        :disabled="false"
        class="absolute top-0 end-0 p-2.5 h-full text-sm font-medium text-white dark:text-black bg-gray-600 rounded-e-md border hover:bg-gray-700 focus:ring-4 focus:outline-none focus:ring-gray-300 dark:bg-gray-200 dark:hover:bg-gray-300 dark:focus:ring-gray-800"
      >
        Search
      </button>
    </div>
  </div>
</template>
