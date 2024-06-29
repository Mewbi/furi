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

    const res = await api.getAnalytics(uri, fromTimestamp, toTimestamp, '1min');
    if ('error' in res) {
      if ('message' in res && res.message)
        return notifyStore.notify(`${res.message}`, NotificationType.Error);
      return notifyStore.notify('An error occured to get uri metrics', NotificationType.Error);
    }

    console.log(res);
    emit('analyticsResult', res);
    notifyStore.notify('Success creating short url', NotificationType.Success);
  };
</script>

<template>
  <div class="mt-8 w-full max-w-md relative mb-4">

    <div class="flex flex-col space-y-4">
      <div class="flex ">
        <label for="fromDate" class="pl-10 w-60 h-full rounded-s-md p-2.5 text-gray-900 bg-gray-300 dark:text-white dark:bg-gray-700"><b>From Date</b></label>
        <input
          type="datetime-local"
          v-model="fromDate"
          id="fromDate"
          class="ps-10 bg-gray-100 text-gray-900 text-sm rounded-e-md w-full p-2.5 dark:bg-gray-600 dark:placeholder-gray-400 dark:text-white font-bold"
        />
      </div>

      <div class="flex ">
        <label for="toDate" class="pl-10 w-60 h-full rounded-s-md p-2.5 text-gray-900 bg-gray-300 dark:text-white dark:bg-gray-700"><b>To Date</b></label>
        <input
          type="datetime-local"
          v-model="toDate"
          id="toDate"
          class="ps-10 bg-gray-100 text-gray-900 text-sm rounded-e-md w-full p-2.5 dark:bg-gray-600 dark:placeholder-gray-400 dark:text-white font-bold"
        />
      </div>

      <div class="flex ">
        <label for="aggregation" class=" pl-10 w-60 h-full rounded-s-md p-2.5 text-gray-900 bg-gray-300 dark:text-white dark:bg-gray-700"><b>Aggregation</b></label>
        <select
          v-model="aggregation"
          id="aggregation"
          class="ps-10 w-full bg-gray-100 text-gray-900 text-sm rounded-e-md p-2.5 dark:bg-gray-600 dark:placeholder-gray-400 dark:text-white font-bold"
        >
          <option value="1min">By minute</option>
          <option value="1hour">By hour</option>
          <option value="1d">By day</option>
        </select>
      </div>

      <div class="flex flex-col space-y-2">
        <button
          @click="getAnalytics"
          :disabled="false"
          class="top-0 end-0 p-2.5 h-full text-sm font-bold text-white dark:text-black bg-gray-600 rounded-md border hover:bg-gray-700 focus:ring-4 focus:outline-none focus:ring-gray-300 dark:bg-gray-200 dark:hover:bg-gray-300 dark:focus:ring-gray-800"
        >
          Search
        </button>
      </div>
    </div>
  </div>
</template>
