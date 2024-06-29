<script setup lang="ts">
  import { use } from 'echarts/core';
  import { CanvasRenderer } from 'echarts/renderers';
  import { PieChart } from 'echarts/charts';
  import { TitleComponent, TooltipComponent, LegendComponent } from 'echarts/components';
  import VChart from 'vue-echarts';
  import { ref } from 'vue';

  use([CanvasRenderer, PieChart, TitleComponent, TooltipComponent, LegendComponent]);

  const props = defineProps({
    data: {
      type: Object,
      required: true,
    },
  });

  let chart = ref();
  if (props.data && Array.isArray(props.data.device_count)) {
    chart = ref({
      title: {
        text: 'Device types',
        subtext: 'Devices that accessed your link',
        left: 'center',
        textStyle: {
          color: '#ffffff',
        },
        subtextStyle: {
          color: '#ffffff',
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
        orient: 'vertical',
        left: 'left',
        bottom: 'bottom',
        textStyle: {
          color: '#ffffff',
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
    });
  }
</script>

<template>
  <v-chart ref="pieChart" :option="chart" class="chart" />
</template>

<style scoped>
  .chart {
    height: 300px;
  }
</style>
