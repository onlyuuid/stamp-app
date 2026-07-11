<template>
  <div class="data-statistics-container">
    <a-row :gutter="20" class="top-cards">
      <a-col :span="8">
        <a-card :bordered="false" class="stat-card total-drops">
          <span class="card-icon">💧</span>
          <div class="card-info">
            <div class="card-label">本周总投入</div>
            <div class="card-value">{{ total }} <span class="unit">滴</span></div>
          </div>
        </a-card>
      </a-col>
      <a-col :span="8">
        <a-card :bordered="false" class="stat-card streak-days">
          <span class="card-icon">🔥</span>
          <div class="card-info">
            <div class="card-label">连续坚持</div>
            <div class="card-value">{{ steakDays }} <span class="unit">天</span></div>
          </div>
        </a-card>
      </a-col>
      <a-col :span="8">
        <a-card :bordered="false" class="stat-card active-bottles">
          <span class="card-icon">🧪</span>
          <div class="card-info">
            <div class="card-label">活跃蓄水瓶</div>
            <div class="card-value">{{ taskList.length }} <span class="unit">个</span></div>
          </div>
        </a-card>
      </a-col>
    </a-row>

    <a-row :gutter="20" class="middle-charts">
      <a-col :span="16">
        <a-card title="水流成长轨迹 (近一周)" :bordered="false" class="chart-card">
          <div ref="lineChartRef" class="echart-box"></div>
        </a-card>
      </a-col>
      
      <a-col :span="8">
        <a-card title="精力水源分布" :bordered="false" class="chart-card">
          <div ref="pieChartRef" class="echart-box"></div>
        </a-card>
      </a-col>
    </a-row>

    <a-row :gutter="20" class="bottom-charts">
      <a-col :span="24">
        <a-card title="年度能量蓄水热力墙" :bordered="false" class="chart-card">
          <div ref="heatmapChartRef" class="echart-box heatmap-box"></div>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue';
import * as echarts from 'echarts';
import {week_total,streak_days,get_heatmap} from '../../api/water_log'
import { listTask,weeklyStats } from '../../api/task'

const total = ref(0);
const steakDays = ref(0);
const taskNum = ref(0);
const taskList = ref([])

const pieChartDate = ref([])
const taskNameData = ref([])


const init = async () => {
  /** 本周总投入 */
  total.value = await week_total();
  /** 连续坚持天数 */
  steakDays.value = await streak_days();
  /** 任务数 */
  await getTaskList();
   
}

/** 获取任务列表 */
const getTaskList = async () => {
  taskList.value = await listTask()
  
  // 加载饼图
  initPieChart();
  
  // 封装折线图数据
  // 1.任务名列表
  taskNameData.value = taskList.value.map(item => item.taskName)
  console.log('taskNameData.value',taskNameData.value)
  
  const lineChartData = await weeklyStats();
  initLineChart(lineChartData)

  let year = new Date().getFullYear();
  const virtualData = await get_heatmap(year);
  console.log('virtualData',virtualData)
  // 将数据封装转换成 ECharts 日历热力图需要的格式：[['2026-01-01', 0], ...]
  const chartData = virtualData.map(item => [
    item.date,        // 对应二维数组第 1 项：日期字符串
    item.total_water  // 对应二维数组第 2 项：注入的水滴数
  ]);
  initHeatmapChart(chartData)
}; 

let colors = [{ color: '#439671' },{ color: '#3b82f6' },{ color: '#8b5cf6' },{ color: '#d97706' }]

// 图表 DOM 引用
const lineChartRef = ref(null);
const pieChartRef = ref(null);
const heatmapChartRef = ref(null);

// 图表实例引用
let lineChart = null;
let pieChart = null;
let heatmapChart = null;

// 初始化折线图
const initLineChart = (lineChartData) => {
  
  if(lineChartData.length == 0)return;
   
  const seriesData = lineChartData.map(item => {
  
  
  return {
    name: item.task_name, // 对应 ECharts 的图例名称
    type: 'line',
    stack: 'Total',       // 如果不需要堆叠，可以把这行删掉
    smooth: true,
    areaStyle: { 
      opacity: 0.3 // 自动产生半透明的面积阴影
    },
    // itemStyle: { color: '#439671' }, 
    data: item.daily_water // 将后端的 7 天数组赋值给 data
  };
});

  if (!lineChartRef.value) return;
  lineChart = echarts.init(lineChartRef.value);
  
  const option = {
    tooltip: { trigger: 'axis', axisPointer: { type: 'cross' } },
    legend: { data: taskNameData.value, bottom: 0 },
    grid: { left: '3%', right: '4%', top: '10%', bottom: '15%', containLabel: true },
    xAxis: { type: 'category', boundaryGap: false, data: ['周一', '周二', '周三', '周四', '周五', '周六', '周日'] },
    yAxis: { type: 'value', name: '单位 (滴)' },
    series: seriesData
  };
  lineChart.setOption(option);
};

// 初始化饼图
const initPieChart = () => {

  taskNum.value = taskList.value.length;
  console.log('taskList',taskList.value)
  
  // 封装饼图数据
  for(let index in  taskList.value){
    console.log('index',index)
    let data = {
      value: taskList.value[index].addUpNum,
      name: taskList.value[index].taskName,
      itemStyle: colors[index]
    }

    pieChartDate.value.push(data);
    console.log('pieChartDate.value',pieChartDate.value)
  }

  if (!pieChartRef.value) return;
  pieChart = echarts.init(pieChartRef.value);
  
  const option = {
    tooltip: { trigger: 'item', formatter: '{b} : {c} 滴 ({d}%)' },
    legend: { bottom: '0', left: 'center', itemWidth: 10, itemHeight: 10 },
    series: [
      {
        name: '水源分配',
        type: 'pie',
        radius: ['45%', '70%'],
        avoidLabelOverlap: false,
        itemStyle: { borderRadius: 8, borderColor: '#fff', borderWidth: 2 },
        label: { show: false, position: 'center' },
        emphasis: { label: { show: true, fontSize: '16', fontWeight: 'bold' } },
        labelLine: { show: false },
        data: pieChartDate.value
      }
    ]
  };
  pieChart.setOption(option);
};


// 初始化热力图
const initHeatmapChart = (virtualData) => {
  if (!heatmapChartRef.value) return;

  heatmapChart = echarts.init(heatmapChartRef.value);
  const option = {
    tooltip: { position: 'top', formatter: (p) => `${p.data[0]} : 注入 ${p.data[1]} 滴水` },
    visualMap: {
        min: 0, 
        max: 60, 
        type: 'piecewise', 
        orient: 'horizontal', 
        left: 'center', 
        top: 0,            // 保持在最顶部
        itemGap: 12,       // 适当缩小图例文字间距，防止横向拉伸撑开
        pieces: [
          { min: 0, max: 8, label: '干燥', color: '#ebedf0' },
          { min: 8, max: 12, label: '微润', color: '#c6e48b' },
          { min: 12, max: 16, label: '蓄水', color: '#7bc96f' },
          { min: 16, max: 20, label: '充沛', color: '#239a3b' },
          { min: 20, max: 24, label: '涌动', color: '#196127' }
        ]
    },
    calendar: {
      top: 70, bottom: 0, left: 40, right: 10,
      range: new Date().getUTCFullYear(), // 固定展示2026年,
      cellSize: ['auto', 13],
      splitLine: { show: false },
      itemStyle: { borderWidth: 2, borderColor: '#fff' },
      yearLabel: { show: false },
      // ----------------- 新增中文坐标轴配置 -----------------
      dayLabel: {
        nameMap: 'cn',
        firstDay: 1, // 星期一排在第一列
        nameMap: ['周 日', '周 一', '周 二', '周 三', '周 四', '周 五', '周 六'] // 纵坐标星期改为中文
      },
      monthLabel: {
        nameMap: [
          '一月', '二月', '三月', '四月', '五月', '六月', 
          '七月', '八月', '九月', '十月', '十一月', '十二月'
        ] // 横坐标月份改为中文
      }
    },
    series: { type: 'heatmap', coordinateSystem: 'calendar', data: virtualData }
  };
  heatmapChart.setOption(option);
};

// 监听窗口缩放，防止图表变形
const handleResize = () => {
  lineChart?.resize();
  pieChart?.resize();
  heatmapChart?.resize();
};

onMounted(() => {
  init();

  // initHeatmapChart();
  window.addEventListener('resize', handleResize);
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize);
  lineChart?.dispose();
  pieChart?.dispose();
  heatmapChart?.dispose();
});
</script>

<style lang="scss" scoped>
.data-statistics-container {
    background-color: #f8fafc;
    width: 100%;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 15px;
    flex: 1;
    overflow: hidden;
   /* 核心：禁止文本被选中 */
    user-select: none; 
    /* 兼容旧版本浏览器的写法（可选） */
    -webkit-user-select: none; /* Safari */
    -moz-user-select: none;    /* Firefox */
    -ms-user-select: none;     /* IE10+ */
//   border: 1px solid red;
  /* 顶部大字报卡片样式 */
  .top-cards {
    .stat-card {
      border-radius: 16px;
      padding: 10px;
      box-shadow: 0 4px 20px rgba(148, 163, 184, 0.06);
      transition: transform 0.2s;
      
      &:hover { transform: translateY(-2px); }

      :deep(.ant-card-body) {
        display: flex;
        align-items: center;
        gap: 20px;
      }

      .card-icon {
        font-size: 32px;
        width: 56px;
        height: 56px;
        border-radius: 14px;
        display: flex;
        align-items: center;
        justify-content: center;
      }

      .card-info {
        .card-label { font-size: 14px; color: #64748b; margin-bottom: 4px; }
        .card-value {
          font-size: 28px; font-weight: 700; color: #1e293b; line-height: 1;
          .unit { font-size: 14px; font-weight: 500; color: #94a3b8; margin-left: 2px; }
        }
      }
    }

    /* 各卡片特有色彩主题 */
    .total-drops .card-icon { background: rgba(59, 130, 246, 0.1); }
    .streak-days .card-icon { background: rgba(239, 68, 68, 0.1); }
    .active-bottles .card-icon { background: rgba(16, 185, 129, 0.1); }
  }

  /* 图表基础卡片包裹 */
  .chart-card {
    border-radius: 20px;
    box-shadow: 0 4px 20px rgba(148, 163, 184, 0.06);
    
    :deep(.ant-card-head) {
      border-bottom: none;
      padding-top: 16px;
      .ant-card-head-title { font-size: 16px; font-weight: 600; color: #334155; }
    }
  }

  /* ECharts 绘图容器盒子 */
  .echart-box {
    width: 100%;
    height: 280px;
    overflow-x: hidden;
  }
  
  .heatmap-box {
    height: 200px;
  }
}
</style>