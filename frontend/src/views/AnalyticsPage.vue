<script setup lang="ts">
  import FooterComponent from '../components/core/FooterComponent.vue';
  import AnalyticsSearch from '../components/core/AnalyticsSearch.vue';
  import { RouterLink } from 'vue-router';

  // import GeoChart from '../components/charts/GeoChart.vue';
  import { use } from 'echarts/core';
  import { CanvasRenderer } from 'echarts/renderers';
  import { LineChart } from 'echarts/charts';
  import {
    TooltipComponent,
    LegendComponent,
    GridComponent,
    ToolboxComponent,
    BrushComponent,
    DataZoomComponent,
    MarkLineComponent,
  } from 'echarts/components';
  import VChart from 'vue-echarts';

  use([
    CanvasRenderer,
    LineChart,
    TooltipComponent,
    LegendComponent,
    GridComponent,
    ToolboxComponent,
    BrushComponent,
    DataZoomComponent,
    MarkLineComponent,
  ]);

  const chart = {
    color: ['#18181B', 'orange'],
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'line',
      },
    },
    legend: {
      left: 0,
      height: '140',
      textStyle: { fontSize: 18 },
      icon: 'circle',
    },
    xAxis: [
      {
        type: 'time',
        boundaryGap: false,
        data: [100, 110, 120],
      },
    ],
    yAxis: [
      {
        type: 'value',
      },
    ],
    series: [
      {
        name: 'Requisições legítimas',
        type: 'line',
        stack: 'Total',
        showSymbol: false,
        areaStyle: {},
        data: [10, 20, 30],
      },
    ],
    dataZoom: [],
    toolbox: {
      right: 0,
      feature: {
        dataZoom: {
          show: true,
          yAxisIndex: false,
          icon: {
            zoom: 'blank',
            back: 'blank',
          },
        },
        restore: {
          show: true,
          title: 'Resetar zoom',
        },
        brush: {
          show: false,
        },
      },
    },
  };
</script>

<template>
  <div
    class="flex flex-col justify-between w-full h-full min-h-screen bg-white dark:bg-gray-800 p-8"
  >
    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full">
      <div class="w-full max-w-lg flex flex-col items-center">
        <RouterLink :to="{ name: 'home' }">
          <picture>
            <source srcset="/logo_light.png" media="(prefers-color-scheme: dark)" />
            <img src="/logo_dark.png" class="w-40 h-40 mb-6" />
          </picture>
        </RouterLink>
        
        <AnalyticsSearch />
      </div>
      

    </div>

    <div class="flex flex-col md:flex-row md:gap-x-2 items-center justify-center h-full">
      <div class="dark:text-white w-full max-w-md text-wrap space-y-2 pl-1 pr-1">
        <p>Boianasdkasjdf</p>
        <v-chart ref="lineChart" :options="chart" class="w-full h-full" />
      </div>
    </div>

    <!-- Footer section -->
    <div class="flex flex-col items-center text-center">
      <FooterComponent />
    </div>
  </div>
</template>

<style scoped></style>
