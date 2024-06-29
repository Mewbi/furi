<script setup lang="ts">
  import * as echarts from 'echarts';
  import { use } from 'echarts/core';
  import { CanvasRenderer } from 'echarts/renderers';
  import { LineChart } from 'echarts/charts';
  import { TitleComponent, TooltipComponent, LegendComponent } from 'echarts/components';
  import VChart from 'vue-echarts';
  import { ref } from 'vue';

  use([CanvasRenderer, LineChart, TitleComponent, TooltipComponent, LegendComponent]);

  const props = defineProps({
    data: {
      type: Object,
      required: true,
    },
  });

  let chart = ref();
  if (props.data && Array.isArray(props.data)) {
    chart = ref({
      color: ['#32c9db'],
      title: {
        text: 'Requests',
        subtext: 'Amount of requests that accessed your link',
        left: 'center',
        textStyle: {
          color: '#ffffff',
        },
        subtextStyle: {
          color: '#ffffff',
        },
      },
      tooltip: {
        trigger: 'axis',
        axisPointer: {
          label: {
            backgroundColor: '#6a7985',
          },
        },
      },
      legend: {
        orient: 'vertical',
        left: 'left',
        bottom: 'bottom',
        textStyle: {
          color: '#ffffff',
        },
      },
      toolbox: {
        feature: {
          saveAsImage: {},
        },
      },
      xAxis: [
        {
          type: 'category',
          boundaryGap: false,
          data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
        },
      ],
      yAxis: [
        {
          type: 'value',
        },
      ],
      series: [
        {
          name: 'Requests',
          type: 'line',
          stack: 'Total',
          smooth: true,
          lineStyle: {
            width: 0,
          },
          showSymbol: false,
          areaStyle: {
            opacity: 0.8,
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              {
                offset: 0,
                color: 'rgb(128, 255, 165)',
              },
              {
                offset: 1,
                color: 'rgb(1, 191, 236)',
              },
            ]),
          },
          emphasis: {
            focus: 'series',
          },
          data: [140, 232, 101, 264, 90, 340, 250],
        },
      ],
    });
  }
</script>

<template>
  <v-chart ref="lineChart" :option="chart" class="chart h-full w-full" />
</template>

<style scoped>
  .chart {
    height: 300px;
  }
</style>
