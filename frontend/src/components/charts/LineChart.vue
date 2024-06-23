<script>
  // TODO: Change it to use lang Typescript
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { LineChart } from "echarts/charts";
import {
  TooltipComponent,
  LegendComponent,
  GridComponent,
  ToolboxComponent,
  BrushComponent,
  DataZoomComponent,
  MarkLineComponent,
} from "echarts/components";
import VChart from "vue-echarts";

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

export default {
  name: "LineChart",
  components: {
    VChart,
  },

  props: ["options"],

  data() {
    return { emitRestore: true };
  },

  computed: {
    chartOptions() {
      return {
        ...this.options,

        brush: {
          xAxisIndex: "all",
          outOfBrush: {
            colorAlpha: 0.1,
          },
        },

        grid: {
          top: 100,
          left: "10px",
          right: "3%",
          containLabel: true,
          tooltip: {
            show: true,
            axisPointer: {
              type: "line",
            },
          },
        },
      };
    },
  },

  mounted() {
    setTimeout(() => {
      this.enableSelectToZoom();
    }, 200);
  },
  methods: {
    enableSelectToZoom(forcedReset = false) {
      const lineChart = this.$refs.lineChart;
      lineChart.dispatchAction({
        type: "takeGlobalCursor",
        key: "dataZoomSelect",
        dataZoomSelectActive: true,
      });

      if (forcedReset) {
        if (this.emitRestore) return this.$emit("resetTime");

        this.emitRestore = true;
      }
    },
    emitZoom(event) {
      if (!event.batch) return;

      this.$emit("zoom", {
        start: parseInt(event.batch[0].startValue, 10),
        end: parseInt(event.batch[0].endValue, 10),
      });
    },
    restoreChart() {
      this.emitRestore = false;

      const interval = setInterval(() => {
        if (this.$refs.lineChart) {
          this.$refs.lineChart.dispatchAction({
            type: "restore",
          });
          clearInterval(interval);
        }
      }, 50);
    },
  },
};
</script>

<template>
  <v-chart
    class="chart"
    :option="chartOptions"
    ref="lineChart"
    @restore="enableSelectToZoom(true)"
    @datazoom="emitZoom"
    autoresize
  />
</template>

<style scoped>
.chart {
  height: 100vh;
}
</style>
