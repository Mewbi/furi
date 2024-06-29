<script setup lang="ts">
  import FooterComponent from '../components/core/FooterComponent.vue';
  import AnalyticsSearch from '../components/core/AnalyticsSearch.vue';
  import { RouterLink } from 'vue-router';
  import GeoChart from '../components/charts/GeoChart.vue';
  import LineChart from '../components/charts/LineChart.vue';
  import PieChart from '../components/charts/PieChart.vue';
  import { UriMetricsResponse } from '../services/api';
  import { useRouter } from 'vue-router';

  const router = useRouter();
  const uri = router.currentRoute.value.params.id as string;

  const data = {
    device_count: [
      { device_type: 'Type A', count: 10 },
      { device_type: 'Type B', count: 20 },
      // Add more data points as needed
    ]
  };

  const time_series = {
    data: [
      { date: 1719259709, count: 10 },
      { date: 1719259720, count: 20 },
      // Add more data points as needed
    ]
  };

  const populateData = (data: UriMetricsResponse) => {
    console.log(data);
  }
</script>

<template>
  <div
    class="flex flex-col justify-between w-full h-full min-h-screen bg-white dark:bg-gray-800 p-8"
  >
    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full mb-6">
      <div class="w-full max-w-lg flex flex-col items-center">
        <RouterLink :to="{ name: 'home' }">
          <picture>
            <source srcset="/logo_light.png" media="(prefers-color-scheme: dark)" />
            <img src="/logo_dark.png" class="w-40 h-40 mb-6" />
          </picture>
        </RouterLink>

        <h1 class="text-gray-900 dark:text-white text-2xl text-wrap font-bold">Analytics Metrics of: {{ uri }}</h1>
        
        <AnalyticsSearch @analyticsResult = "populateData" />
      </div>
    </div>

    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full">
      <div class="dark:text-white w-full max-w-md text-wrap space-y-2 pl-1 pr-1">
        <LineChart :data="time_series.data"/>
      </div>
    </div>

    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full">
      <div class="dark:text-white w-full max-w-md text-wrap space-y-2 pl-1 pr-1">
        <GeoChart />
      </div>
    </div>

    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full">
      <div class="dark:text-white w-full max-w-md text-wrap space-y-2 pl-1 pr-1">
        <PieChart :data="data"/>
      </div>
    </div>

    <!-- Footer section -->
    <div class="flex flex-col items-center text-center">
      <FooterComponent />
    </div>
  </div>
</template>

<style scoped></style>
