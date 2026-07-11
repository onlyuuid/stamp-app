<template>
  <div class="calendar-view-container">
    <a-card :bordered="false" class="calendar-card">
      <template #title>
        <div class="calendar-header-title">
          <span class="icon">📅</span> 历史点滴灌溉看板
        </div>
      </template>

      <a-calendar v-model:value="selectedDate" :fullscreen="true">
        <template #dateCellRender="{ current }">
          <div class="date-cell-content" :class="getWaterLevelClass(current)">
            <template v-if="getTodayData(current)">
              <a-popover trigger="hover" placement="top">
                <template #content>
                  <div class="popover-drop-detail">
                    <div class="popover-title">💧 {{ current.format('YYYY-MM-DD') }}</div>
                    <div class="popover-divider"></div>
                    <div class="popover-item" v-for="task in getTodayData(current).tasks" :key="task.id">
                      <span class="dot" :style="{ backgroundColor: task.color }"></span>
                      <span class="name">{{ task.name }}</span>
                      <span class="count">+{{ task.drops }} 滴</span>
                    </div>
                    <div class="popover-total">当日共计：{{ getTodayData(current).total }} 滴</div>
                  </div>
                </template>
                
                <div class="water-badge">
                  <span class="drop-icon">💧</span>
                  <span class="drop-count">{{ getTodayData(current).total }}</span>
                </div>
              </a-popover>
            </template>
          </div>
        </template>
      </a-calendar>
    </a-card>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import dayjs from 'dayjs';

// 当前选中的日期（默认今天）
const selectedDate = ref(dayjs());

/** * 1. 模拟从 localStorage 中读取的打卡历史数据 
 * 结构：以 'YYYY-MM-DD' 为键，存储当天的任务注入详情
 */
const mockHistoryData = ref({
  '2026-07-08': {
    total: 45,
    tasks: [
      { id: 1, name: '基酒系统可视化大屏', drops: 15, color: '#439671' },
      { id: 2, name: '考研英语单词', drops: 30, color: '#8b5cf6' }
    ]
  },
  '2026-07-07': {
    total: 12,
    tasks: [
      { id: 3, name: 'STM32底层驱动编写', drops: 12, color: '#d97706' }
    ]
  },
  '2026-07-05': {
    total: 80,
    tasks: [
      { id: 1, name: '基酒系统可视化大屏', drops: 50, color: '#439671' },
      { id: 4, name: '心语漫漫UI调整', drops: 30, color: '#3b82f6' }
    ]
  }
});

// 获取某一天对应的打卡数据
const getTodayData = (currentDate) => {
  const dateStr = currentDate.format('YYYY-MM-DD');
  return mockHistoryData.value[dateStr] || null;
};

// 根据当天注入的水滴总数，动态赋予格子不同的治愈系青绿色背景
const getWaterLevelClass = (currentDate) => {
  const data = getTodayData(currentDate);
  if (!data) return 'level-dry';
  if (data.total <= 15) return 'level-moist';   // 微润
  if (data.total <= 40) return 'level-gather';  // 蓄水
  return 'level-abundant';                      // 充沛
};
</script>

<style lang="scss" scoped>
.calendar-view-container {
  background-color: #f8fafc;
  height: calc(100vh - 150px);
  width: 100%;
  box-sizing: border-box;
  .calendar-card {
    border-radius: 20px;
    box-shadow: 0 4px 20px rgba(148, 163, 184, 0.06);
    overflow: hidden;
    // border: 1px solid red;
    
    .calendar-header-title {
      font-size: 16px;
      font-weight: 600;
      color: #334155;
      display: flex;
      align-items: center;
      gap: 8px;
    }
  }

  /* 针对 Ant Design 日历单元格的深度魔改样式 */
  :deep(.ant-picker-calendar-date) {
    border-top: 2px solid #f1f5f9 !important;
    margin: 0 !important;
    padding: 4px !important;
    min-height: 90px !important;
    max-height: 108px;
    transition: background-color 0.2s;

    &:hover {
      background-color: #f1f5f9;
    }
  }

  /* 选中的日期单元格高亮外框 */
  :deep(.ant-picker-calendar-date-value) {
    color: #475569;
    font-weight: 600;
  }

  :deep(.ant-picker-cell-selected .ant-picker-calendar-date) {
    background-color: rgba(59, 130, 246, 0.04);
    .ant-picker-calendar-date-value {
      color: #3b82f6;
    }
  }

  /* 日期格子内容容器 */
  .date-cell-content {
    width: 100%;
    height: calc(100% - 24px);
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    border-radius: 6px;
    padding: 4px;
    // border: 1px solid red;
  }


  /* 🌟 水滴能量等级色彩体系（匹配你的热力图概念） */
  .level-dry { background-color: transparent; }
  .level-moist { background-color: rgba(198, 228, 139, 0.25); }
  .level-gather { background-color: rgba(123, 201, 111, 0.25); }
  .level-abundant { background-color: rgba(35, 154, 59, 0.15); }

  /* 格子内部的小徽章 */
  .water-badge {
    display: flex;
    align-items: center;
    gap: 2px;
    background: white;
    padding: 2px 6px;
    border-radius: 20px;
    width: fit-content;
    box-shadow: 0 2px 8px rgba(148, 163, 184, 0.12);
    border: 1px solid rgba(148, 163, 184, 0.1);
    cursor: pointer;
    transform: scale(0.9);
    transform-origin: left bottom;

    .drop-icon { font-size: 12px; }
    .drop-count { font-size: 11px; font-weight: 700; color: #3b82f6; }
  }
}

/* 弹出悬浮窗气泡样式 */
.popover-drop-detail {
  padding: 4px;
  min-width: 180px;
  .popover-title {
    font-size: 13px;
    font-weight: 600;
    color: #475569;
  }

  .popover-divider {
    height: 1px;
    background-color: #f1f5f9;
    margin: 8px 0;
  }

  .popover-item {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 6px;
    font-size: 12px;

    .dot {
      width: 8px;
      height: 8px;
      border-radius: 50%;
      flex-shrink: 0;
    }
    .name {
      color: #64748b;
      margin-right: auto;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      max-width: 110px;
    }
    .count {
      font-weight: 600;
      color: #334155;
    }
  }

  .popover-total {
    margin-top: 8px;
    font-size: 12px;
    font-weight: 600;
    color: #3b82f6;
    text-align: right;
  }
}
</style>