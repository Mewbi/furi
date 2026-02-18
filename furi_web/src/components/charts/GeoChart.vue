<script setup lang="ts">
  import { use, registerMap } from 'echarts/core';
  import { ScatterChart, EffectScatterChart } from 'echarts/charts';
  import {
    GeoComponent,
    TitleComponent,
    LegendComponent,
    TooltipComponent,
  } from 'echarts/components';
  import { computed, shallowRef, ref, onMounted, onUnmounted } from 'vue';

  const isDark = ref(document.documentElement.classList.contains('dark'));
  const observer = new MutationObserver(() => {
    isDark.value = document.documentElement.classList.contains('dark');
  });
  onMounted(() => observer.observe(document.documentElement, { attributeFilter: ['class'] }));
  onUnmounted(() => observer.disconnect());
  import VChart from 'vue-echarts';
  import worldMap from '../../assets/world.json';
  import { countryCodeToName } from '../../utils/countries.ts';

  use([
    ScatterChart,
    EffectScatterChart,
    GeoComponent,
    TitleComponent,
    LegendComponent,
    TooltipComponent,
  ]);

  // @ts-expect-error: Json has no notation
  registerMap('world', worldMap);

  const props = defineProps<{
    data: Array<{ country: string; count: number }>;
  }>();

 
  const itemStyle = {
    normal: {
      borderWidth: 0.5,
      borderColor: 'black',
    },
    emphasis: {
      label: { show: true },
      shadowOffsetX: 0,
      shadowOffsetY: 0,
      shadowBlur: 20,
      shadowColor: 'rgba(0, 0, 0, 0.3)',
    },
  };

  const chart = computed(() => {
  if (!props.data || !Array.isArray(props.data)) {
      return {};
    }

    const chartData = props.data.map(d => ({
      name: countryCodeToName[d.country] || d.country,
      value: d.count,
    }));

    const maxValue = Math.max(...props.data.map(d => d.count), 1);

    const textColor = isDark.value ? '#ffffff' : '#111827';

    return {
      title: {
        text: 'World Access',
        subtext: 'Access around the world based in Geo IP localization',
        left: 'center',
        top: 'top',
        textStyle: {
          color: textColor,
        },
        subtextStyle: {
          color: textColor,
        },
      },
      toolbox: {
        feature: {
          saveAsImage: {},
        },
      },
      tooltip: {
        trigger: 'item',
        formatter: (params: { value: number; name: string; seriesName: string }) => {
          return params.seriesName + '<br/>' + params.name + ' : ' + (params.value || 0);
        },
      },
      visualMap: {
        min: 0,
        max: maxValue,
        realtime: true,
        show: false,
        color: ['#058796', '#32c9db', '#69efff'],
      },
      series: [
        {
          name: 'Country access',
          type: 'map',
          map: 'world',
          roam: true,
          top: 60,
          width: '80%',
          label: {
            textBorderColor: '#ffffff',
            textBorderWidth: 1,
          },
          itemStyle: itemStyle,
          data: chartData,
        },
      ],
    };
  });

  const map = shallowRef(null);
</script>

<template>
  <v-chart class="chart" ref="map" :option="chart" autoresize />
</template>

<style scoped>
  .chart {
    height: 300px;
  }

  @media (min-width: 768px) {
    .chart {
      height: 450px;
    }
  }

  @media (min-width: 1024px) {
    .chart {
      height: 550px;
    }
  }
</style>
