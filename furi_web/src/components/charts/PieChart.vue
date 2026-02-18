<script setup lang="ts">
  import { use } from 'echarts/core';
  import { CanvasRenderer } from 'echarts/renderers';
  import { PieChart } from 'echarts/charts';
  import { TitleComponent, TooltipComponent, LegendComponent } from 'echarts/components';
  import VChart from 'vue-echarts';
  import { computed, ref, onMounted, onUnmounted } from 'vue';

  const isDark = ref(document.documentElement.classList.contains('dark'));
  const observer = new MutationObserver(() => {
    isDark.value = document.documentElement.classList.contains('dark');
  });
  onMounted(() => observer.observe(document.documentElement, { attributeFilter: ['class'] }));
  onUnmounted(() => observer.disconnect());

  use([CanvasRenderer, PieChart, TitleComponent, TooltipComponent, LegendComponent]);

  const props = defineProps<{
    data: {
      device_count: Array<{ device_type: string; count: number }>;
    };
  }>();

  const chart = computed(() => {
    if (!props.data || !Array.isArray(props.data.device_count)) {
      return {};
    }

    const textColor = isDark.value ? '#ffffff' : '#111827';

    return {
      title: {
        text: 'Device types',
        subtext: 'Devices that accessed your link',
        left: 'center',
        textStyle: {
          color: textColor,
        },
        subtextStyle: {
          color: textColor,
        },
      },
      tooltip: {
        trigger: 'item',
      },
      toolbox: {
        feature: {
          saveAsImage: {},
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
      series: [
        {
          name: 'Device',
          type: 'pie',
          radius: ['40%', '70%'],
          avoidLabelOverlap: false,
          padAngle: 5,
          itemStyle: {
            borderRadius: 10,
          },
          data: props.data.device_count.map((d) => ({ name: d.device_type, value: d.count })),
          emphasis: {
            itemStyle: {
              shadowBlur: 10,
              shadowOffsetX: 0,
              shadowColor: 'rgba(0, 0, 0, 0.5)',
            },
          },
        },
      ],
    };
  });
</script>

<template>
  <v-chart ref="pieChart" :option="chart" class="chart w-full" autoresize />
</template>

<style scoped>
  .chart {
    height: 280px;
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
