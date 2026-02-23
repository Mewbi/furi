<script setup lang="ts">
  import { ref } from 'vue';
  import api, { UriMetricsResponse } from '../../services/api';
  import { NotificationType, useNotifyStore } from '../../stores/notifyStore';
  import { useRouter } from 'vue-router';

  const router = useRouter();
  const uri = router.currentRoute.value.params.id as string;

  const notifyStore = useNotifyStore();

  const emit = defineEmits<{ analyticsResult: [UriMetricsResponse] }>();

  const now = new Date();
  const yesterday = new Date(now);
  yesterday.setDate(yesterday.getDate() - 1);

  const formatDate = (date: Date): string  => {
    return date.toISOString().slice(0, 16)
  }

  const fromDate = ref(formatDate(yesterday));
  const toDate = ref(formatDate(now));
  const aggregation = ref('1min');

  const getAnalytics = async () => {

    const fromTimestamp = new Date(fromDate.value).getTime() / 1000;
    const toTimestamp = new Date(toDate.value).getTime() / 1000;

    const res = await api.getAnalytics(uri, fromTimestamp, toTimestamp, aggregation.value);
    if ('error' in res) {
      if ('message' in res && res.message)
        return notifyStore.notify(`${res.message}`, NotificationType.Error);
      return notifyStore.notify('An error occured to get uri metrics', NotificationType.Error);
    }

    emit('analyticsResult', res);
    notifyStore.notify('Success fetching metrics', NotificationType.Success);
  };

  defineExpose({ getAnalytics });
</script>

<template>
  <div class="mt-8 w-full max-w-md">
    <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 shadow-sm p-5">

      <h2 class="text-base font-semibold text-gray-900 dark:text-white mb-4">Query range</h2>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div class="flex flex-col gap-1">
          <label for="fromDate" class="text-sm font-medium text-gray-700 dark:text-gray-300">From</label>
          <input type="datetime-local" v-model="fromDate" id="fromDate"
            class="bg-gray-100 dark:bg-gray-600 text-gray-900 dark:text-white text-sm rounded-md p-2.5
                   border border-gray-300 dark:border-gray-500
                   focus:outline-none focus:ring-2 focus:ring-gray-400 dark:focus:ring-gray-500" />
        </div>
        <div class="flex flex-col gap-1">
          <label for="toDate" class="text-sm font-medium text-gray-700 dark:text-gray-300">To</label>
          <input type="datetime-local" v-model="toDate" id="toDate"
            class="bg-gray-100 dark:bg-gray-600 text-gray-900 dark:text-white text-sm rounded-md p-2.5
                   border border-gray-300 dark:border-gray-500
                   focus:outline-none focus:ring-2 focus:ring-gray-400 dark:focus:ring-gray-500" />
        </div>
      </div>

      <div class="flex flex-col gap-1 mt-3">
        <label for="aggregation" class="text-sm font-medium text-gray-700 dark:text-gray-300">Aggregation</label>
        <select v-model="aggregation" id="aggregation"
          class="bg-gray-100 dark:bg-gray-600 text-gray-900 dark:text-white text-sm rounded-md p-2.5
                 border border-gray-300 dark:border-gray-500
                 focus:outline-none focus:ring-2 focus:ring-gray-400 dark:focus:ring-gray-500">
          <option value="1min">By minute</option>
          <option value="1hour">By hour</option>
          <option value="1day">By day</option>
        </select>
      </div>

      <button @click="getAnalytics" class="mt-4 w-full p-2.5 text-sm font-bold text-white dark:text-black
        bg-gray-600 rounded-md hover:bg-gray-700 focus:ring-4 focus:outline-none focus:ring-gray-300
        dark:bg-gray-200 dark:hover:bg-gray-300 dark:focus:ring-gray-800 transition-colors">
        Search
      </button>

    </div>
  </div>
</template>
