<script setup lang="ts">
  import { NotificationType, useNotifyStore } from '../../stores/notifyStore';
  import { storeToRefs } from 'pinia';

  const notifyStore = useNotifyStore();
  const { notifications } = storeToRefs(notifyStore);
</script>
<template>
  <div class="flex flex-col space-y-4">
    <TransitionGroup name="fade">
      <div v-for="notification in notifications" :key="notification.id">
        <div
          v-if="notification.type === NotificationType.Success"
          class="flex items-center w-screen max-w-md p-3 text-gray-700 bg-white dark:text-white dark:bg-gray-700 dark:border-gray-800 rounded-lg border shadow-2xl "
          role="alert"
        >
          <div
            class="inline-flex items-center justify-center flex-shrink-0 w-8 h-8 text-green-500 bg-green-100 rounded-lg"
          >
            <svg
              class="w-5 h-5"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="currentColor"
              viewBox="0 0 20 20"
            >
              <path
                d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 8.207-4 4a1 1 0 0 1-1.414 0l-2-2a1 1 0 0 1 1.414-1.414L9 10.586l3.293-3.293a1 1 0 0 1 1.414 1.414Z"
              />
            </svg>
            <span class="sr-only">Check icon</span>
          </div>
          <div class="ms-3 text-sm font-normal">{{ notification.text }}</div>
          <button
            type="button"
            class="ms-auto -mx-1.5 -my-1.5 bg-white text-gray-400 hover:text-gray-900 dark:bg-gray-600 dark:text-white dark:hover:bg-gray-400 rounded-lg focus:ring-2 focus:ring-gray-300 p-1.5 hover:bg-gray-100 inline-flex items-center justify-center h-8 w-8"
            data-dismiss-target="#toast-success"
            aria-label="Close"
            @click="notifyStore.clear(notification.id)"
          >
            <span class="sr-only">Close</span>
            <svg
              class="w-3 h-3"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 14 14"
            >
              <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
              />
            </svg>
          </button>
        </div>
        <div
          v-if="notification.type === NotificationType.Error"
          class="flex items-center w-screen max-w-md p-3 text-gray-700 bg-white rounded-lg dark:text-white dark:bg-gray-700 border dark:border-gray-800 shadow-2xl"
          role="alert"
        >
          <div
            class="inline-flex items-center justify-center flex-shrink-0 w-8 h-8 text-red-500 bg-red-100 rounded-lg"
          >
            <svg
              class="w-5 h-5"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="currentColor"
              viewBox="0 0 20 20"
            >
              <path
                d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5Zm3.707 11.793a1 1 0 1 1-1.414 1.414L10 11.414l-2.293 2.293a1 1 0 0 1-1.414-1.414L8.586 10 6.293 7.707a1 1 0 0 1 1.414-1.414L10 8.586l2.293-2.293a1 1 0 0 1 1.414 1.414L11.414 10l2.293 2.293Z"
              />
            </svg>
            <span class="sr-only">Error icon</span>
          </div>
          <div class="ms-3 text-sm font-normal">{{ notification.text }}</div>
          <button
            type="button"
            class="ms-auto -mx-1.5 -my-1.5 bg-white text-gray-400 hover:text-gray-900 dark:bg-gray-600 dark:text-white dark:hover:bg-gray-400 rounded-lg focus:ring-2 focus:ring-gray-300 p-1.5 hover:bg-gray-100 inline-flex items-center justify-center h-8 w-8"
            data-dismiss-target="#toast-danger"
            aria-label="Close"
            @click="notifyStore.clear(notification.id)"
          >
            <span class="sr-only">Close</span>
            <svg
              class="w-3 h-3"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 14 14"
            >
              <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
              />
            </svg>
          </button>
        </div>
        <div
          v-if="notification.type === NotificationType.Warning"
          class="flex items-center w-screen max-w-md p-3 text-gray-700 bg-white dark:text-white dark:bg-gray-700 dark:border-gray-800 rounded-lg border shadow-2xl"
          role="alert"
        >
          <div
            class="inline-flex items-center justify-center flex-shrink-0 w-8 h-8 text-orange-500 bg-orange-100 rounded-lg"
          >
            <svg
              class="w-5 h-5"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="currentColor"
              viewBox="0 0 20 20"
            >
              <path
                d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM10 15a1 1 0 1 1 0-2 1 1 0 0 1 0 2Zm1-4a1 1 0 0 1-2 0V6a1 1 0 0 1 2 0v5Z"
              />
            </svg>
            <span class="sr-only">Warning icon</span>
          </div>
          <div class="ms-3 text-sm font-normal">{{ notification.text }}</div>
          <button
            type="button"
            class="ms-auto -mx-1.5 -my-1.5 bg-white text-gray-400 hover:text-gray-900 dark:bg-gray-600 dark:text-white dark:hover:bg-gray-400 rounded-lg focus:ring-2 focus:ring-gray-300 p-1.5 hover:bg-gray-100 inline-flex items-center justify-center h-8 w-8"
            data-dismiss-target="#toast-warning"
            aria-label="Close"
            @click="notifyStore.clear(notification.id)"
          >
            <span class="sr-only">Close</span>
            <svg
              class="w-3 h-3"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 14 14"
            >
              <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
              />
            </svg>
          </button>
        </div>
      </div>
    </TransitionGroup>
  </div>
</template>
