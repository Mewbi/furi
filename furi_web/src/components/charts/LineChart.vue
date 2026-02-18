<script setup lang="ts">
  import * as echarts from 'echarts';
  import { use } from 'echarts/core';
  import { CanvasRenderer } from 'echarts/renderers';
  import { LineChart } from 'echarts/charts';
  import { TitleComponent, TooltipComponent, LegendComponent } from 'echarts/components';
  import VChart from 'vue-echarts';
  import { computed, ref, onMounted, onUnmounted } from 'vue';

  const isDark = ref(document.documentElement.classList.contains('dark'));
  const observer = new MutationObserver(() => {
    isDark.value = document.documentElement.classList.contains('dark');
  });
  onMounted(() => observer.observe(document.documentElement, { attributeFilter: ['class'] }));
  onUnmounted(() => observer.disconnect());

  use([CanvasRenderer, LineChart, TitleComponent, TooltipComponent, LegendComponent]);

  const props = defineProps<{
    data: Array<{ date: number; count: number }>;
  }>();

  const formatDate = (timestamp: number): string => {
    const date = new Date(timestamp * 1000);
    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
    return `${months[date.getMonth()]}/${date.getDate()} ${date.getHours()}:${date.getMinutes() < 10 ? '0' : ''}${date.getMinutes()}`;
  };

  const chart = computed(() => {
    if (!props.data || !Array.isArray(props.data) || props.data.length === 0) {
      return {};
    }

    const dates = props.data.map(d => formatDate(d.date));
    const counts = props.data.map(d => d.count);
    const textColor = isDark.value ? '#ffffff' : '#111827';

    return {
      color: ['#32c9db'],
      title: {
        text: 'Requests',
        subtext: 'Amount of requests that accessed your link',
        left: 'center',
        textStyle: {
          color: textColor,
        },
        subtextStyle: {
          color: textColor,
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
        orient: 'horizontal',
        left: 'center',
        bottom: 'bottom',
        textStyle: {
          color: textColor,
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
          data: dates,
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
          showSymbol: props.data.length === 1,
          symbolSize: props.data.length === 1 ? 10 : 4,
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
          data: counts,
        },
      ],
    };
  });
</script>

<template>
  <v-chart ref="lineChart" :option="chart" class="chart w-full" autoresize />
</template>

<style scoped>
  .chart {
    height: 250px;
  }

  @media (min-width: 768px) {
    .chart {
      height: 350px;
    }
  }

  @media (min-width: 1024px) {
    .chart {
      height: 400px;
    }
  }
</style>
