<script setup lang="ts">
  import FooterComponent from '../components/core/FooterComponent.vue';
  import AnalyticsSearch from '../components/core/AnalyticsSearch.vue';
  import { RouterLink } from 'vue-router';
  import GeoChart from '../components/charts/GeoChart.vue';
  import LineChart from '../components/charts/LineChart.vue';
  import PieChart from '../components/charts/PieChart.vue';
  import { UriMetricsResponse } from '../services/api';
  import { useRouter } from 'vue-router';
  import { ref, onMounted } from 'vue';

  const router = useRouter();
  const uri = router.currentRoute.value.params.id as string;

  const analyticsData = ref<UriMetricsResponse | null>(null);
  const analyticsSearchRef = ref<InstanceType<typeof AnalyticsSearch> | null>(null);

  const populateData = (data: UriMetricsResponse) => {
    analyticsData.value = data;
  }

  onMounted(() => {
    analyticsSearchRef.value?.getAnalytics();
  });
</script>

<template>
  <div
    class="flex flex-col justify-between w-full h-full min-h-screen bg-white dark:bg-gray-800 p-8"
  >
    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full mb-6">
      <div class="w-full max-w-lg flex flex-col items-center">
        <RouterLink :to="{ name: 'home' }">
          <img src="/logo_dark.png" class="w-40 h-40 mb-6 block dark:hidden" />
          <img src="/logo_light.png" class="w-40 h-40 mb-6 hidden dark:block" />
        </RouterLink>

        <h1 class="text-gray-900 dark:text-white text-2xl text-wrap font-bold">Analytics Metrics of: {{ uri }}</h1>

        <AnalyticsSearch ref="analyticsSearchRef" @analyticsResult = "populateData" />
      </div>
    </div>

    <template v-if="analyticsData">
      <div class="flex flex-col gap-8 w-full max-w-7xl mx-auto px-4">
        <div class="dark:text-white w-full">
          <LineChart :data="analyticsData.req_time_series"/>
        </div>

        <div class="dark:text-white w-full">
          <GeoChart :data="analyticsData.country_access"/>
        </div>

        <div class="dark:text-white w-full max-w-2xl mx-auto">
          <PieChart :data="analyticsData"/>
        </div>
      </div>
    </template>

    <!-- Footer section -->
    <div class="flex flex-col items-center text-center">
      <FooterComponent />
    </div>
  </div>
</template>

<style scoped></style>
