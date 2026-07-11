<template>
    <a-dropdown :trigger="['contextmenu']">
        <div class="card-container">
            <div class="info-section">
            <div class="header-area">
                <div class="icon-wrapper">
                <span class="desktop-icon">💻</span>
                </div>
                <div class="title-group">
                <h2>{{ taskData.taskName }}</h2>
                <p class="subtitle">{{ taskData.description }}</p>
                </div>
            </div>

            <div class="stat-area">
                <div class="stat-label">累计投入</div>
                <div class="count-num">
                    {{ taskData.addUpNum }}
                <span class="unit">滴</span>
                </div>
            </div>
            <div class="day-put">
                <div class="stat-label">今日投入</div>
                <div class="count-num">
                    {{ taskData.dayUpNum }}
                <span class="unit">滴</span>
                </div>
            </div>

            <div class="footer-area">
                <span class="leaf-icon">🍃</span>
                <span class="streak-text">连续投入 
                    <strong class="highlight-days">
                        {{ logs.length }}
                    </strong> 天</span>
            </div>
            </div>
            <div class="visual-section">
            <div class="glass-bottle" @click="addWater">
                <div class="bottle-mouth"></div>
                <div class="bottle-neck"></div>
                <div class="bottle-body">
                <div class="liquid" :style="{height: `${taskData.addUpNum}%`}">
                    <div class="wave wave-back"></div>
                    <div class="wave wave-front"></div>
                    <div class="bubbles">
                    <span class="bubble b1">💧</span>
                    <span class="bubble b2">💧</span>
                    <span class="bubble b3">💧</span>
                    </div>
                </div>
                </div>
            </div>

            <div class="mascot-rabbit">
                <span class="rabbit-emoji">🐰</span>
                <div class="scarf"></div>
            </div>
            
            <div class="bottom-leaves">🌱</div>
            </div>       
        </div>
        <template #overlay>
            <a-menu @click="handleMenuClick">
                <a-menu-item key="edit">编辑任务</a-menu-item>
                <a-popconfirm
                    title="你确定删除这个任务?"
                    ok-text="Yes"
                    cancel-text="No"
                    @confirm="confirmDelete"
                    @cancel="cancelDelete"
                  >
                    <a-menu-item key="delete" danger>删除卡片</a-menu-item>
                </a-popconfirm>
                
            </a-menu>
        </template>
         
  </a-dropdown>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref} from 'vue';
import { message } from 'ant-design-vue';
import { listTask,updateTask,deleteTask } from "../api/task";
import { getLogByTaskId,getLogByDate,deleteLog, createLog,updateLog } from "../api/water_log";
import type { MenuProps } from 'ant-design-vue';
import type { Task } from "../types/task";
import type { WaterLog } from "../types/water_log";

const { taskData } = defineProps<{
   taskData: Task
}>();

const taskList = ref<Task[]>([]);
const logs = ref<WaterLog[]>([]);
const totalPutInto = ref<number>(0);

// 2. 🌟 核心：声明子组件可以触发的事件名
const emit = defineEmits(['refresh','editForm']);


onMounted(() => {
    getTaskList();
    getLogList();
    logByDate();
})



const logForm = reactive<WaterLog>(
  {} as WaterLog
)

/** 查询任务日志 */
const getLogList = async () => {
  // 记录每日日志
  logs.value =  await getLogByTaskId(taskData.taskId); 
}

/**
 * 根据日期查询日志
 */
const logByDate = async () => {
  let date = getToday();
  const logList = await getLogByDate(date);
  logList.forEach(item => {
    totalPutInto.value += item.waterNum;
  })
}



/** 获取任务列表 */
const getTaskList = async () => {
  taskList.value = await listTask();
};

/** 添加水滴 */
const addWater = async () => {
    const today = getToday();   // 假设返回 "2026-07-10" 格式

    // 新的一天重置
    if (taskData.lastRecordDate !== today) {
        taskData.dayUpNum = 0;
        taskData.lastRecordDate = today;
    }

    if (taskData.addUpNum >= 100) {
        message.success('该瓶子已装满');
        return;
    }

    if(taskData.dayUpNum == 24){
      message.warning('今天没有力气投放了, 早点休息, 明日再来');
      return;
    }
    // 1. 更新任务
    taskData.dayUpNum += 1;
    taskData.addUpNum += 1;

    const updatedTask = await updateTask(taskData);

    // 2. 处理日志（核心优化部分）
    logForm.taskId = updatedTask.taskId;
    logForm.waterNum = updatedTask.dayUpNum;
    logForm.createdDate = today;

    try {
        // 查找今天是否已有日志
        const todayLog = logs.value.find(log => log.createdDate === today);

        if (todayLog) {
            // 更新现有日志
            logForm.logId = todayLog.logId;
            await updateLog(logForm);
            console.log('更新今日日志成功');
        } else {
            // 创建新日志
            await createLog(logForm);
            console.log('创建新日志成功');
        }
    } catch (error) {
        console.error('日志操作失败:', error);
        message.error('日志记录失败');
        return;
    }

    emit('refresh');
    message.success('元气 +1 🚀');
};

const handleMenuClick:MenuProps['onClick']  = async ({ key }) => {
  if (key === 'delete') {
    
  }else if(key === 'edit'){
     emit('editForm',taskData);
  }
};

/** 确认删除 */
const confirmDelete = async (e: MouseEvent) => {
   let taskId = taskData.taskId;
    let rowNum = await deleteTask(taskId)
   
    if(rowNum > 0){
      message.success('删除成功');
      await deleteLog(taskId);
      // 🌟 核心：删除成功后，调用 emit 触发父组件的 refresh 方法更新数据
      emit('refresh');
    }
}

/** 取消删除 */
const cancelDelete = (e: MouseEvent) => {

}

/** 获取今天日期 */
const getToday = () => {
    return new Date().toISOString().slice(0, 10)
}


</script>

<style lang="scss" scoped>
/* 外部大卡片容器 */
.card-container {
  width: 38vw;
  height: 380px;
  background: linear-gradient(135deg, #f1f7fd 0%, #f3fbff 100%);
  border-radius: 28px;
  padding: 35px 40px;
  margin: 10px;
  box-shadow: 5px 5px 10px #4d4d4d21;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: relative;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  border: 1px solid rgba(255, 255, 255, 0.8);
  
}

/* ================= 左侧信息样式 ================= */
.info-section {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  height: 100%;
  flex: 1;
}

.header-area {
  display: flex;
  align-items: center;
  gap: 16px;

  .icon-wrapper {
    width: 48px;
    height: 48px;
    background-color: #e3f2ec;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
  
    .desktop-icon {
      font-size: 22px;
      opacity: 0.75;
    }
  }

  .title-group {
    h2 {
      margin: 0;
      font-size: 20px;
      font-weight: 600;
      color: #1e293b;
    }
    .subtitle {
      margin: 4px 0 0 0;
      font-size: 13px;
      color: #64748b;
      letter-spacing: 0.5px;
    }
  }
}

.day-put{
    display: flex;
    align-items: center;
    .stat-label{
        font-size: 14px;
        color: #8294a5;
        margin-top: 8px;
        letter-spacing: 1px;
    }
    .count-num{
        font-size: 2rem;
        font-weight: 700;
        color: #439671; /* 对应图中的治愈系绿色 */
        line-height: 1;
        display: flex;
        align-items: baseline;
    }
    .unit {
      font-size: 16px;
      font-weight: 500;
      color: #439671;
      margin-left: 6px;
    }
}

.stat-area {
  margin-top: 20px;
  display: flex;
  align-items: center;
  .count-num {
    font-size: 4rem;
    font-weight: 700;
    color: #439671; /* 对应图中的治愈系绿色 */
    line-height: 1;
    display: flex;
    align-items: baseline;

    .unit {
      font-size: 16px;
      font-weight: 500;
      color: #439671;
      margin-left: 6px;
    }
  }

  .stat-label {
    font-size: 14px;
    color: #8294a5;
    margin-top: 8px;
    letter-spacing: 1px;
  }
}

.footer-area {
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(67, 150, 113, 0.06);
  padding: 6px 14px;
  border-radius: 20px;
  width: fit-content;

  .leaf-icon {
    font-size: 14px;
  }
  .streak-text {
    font-size: 14px;
    color: #556b60;
    
    .highlight-days {
      font-size: 16px;
      font-weight: 600;
      color: #38805e;
      margin: 0 2px;
    }
  }
}

/* ================= 右侧视觉样式（玻璃瓶+萌宠） ================= */
.visual-section {
  position: relative;
  width: 240px;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: flex-end;
}

/* 纯 CSS 手绘高透玻璃瓶 */
.glass-bottle {
  position: relative;
  width: 200px;
  height: 270px;
  margin-bottom: 15px;
  z-index: 2;
//   border: 1px solid red;
  .bottle-mouth {
    width: 130px;
    height: 12px;
    border: 3px solid #b2ced4;
    border-radius: 6px;
    margin: 0 auto;
    background: rgba(255, 255, 255, 0.6);
  }

  .bottle-neck {
    width: 124px;
    height: 10px;
    border-left: 3px solid #b2ced4;
    border-right: 3px solid #b2ced4;
    margin: -1px auto 0;
    background: rgba(255, 255, 255, 0.4);
  }

  .bottle-body {
    width: 196px;
    height: 238px;
    border: 3px solid #b2ced4;
    border-radius: 40px 40px 32px 32px;
    margin-top: -1px;
    position: relative;
    overflow: hidden;
    background: rgba(255, 255, 255, 0.25);
    box-shadow: inset 0 0 16px rgba(178, 206, 212, 0.4), 
                0 8px 20px rgba(0, 0, 0, 0.02);

    /* 左右侧高光反射，营造玻璃质感 */
    &::before {
      content: '';
      position: absolute;
      top: 20px;
      left: 8px;
      width: 6px;
      height: 140px;
      background: rgba(255, 255, 255, 0.5);
      border-radius: 3px;
      z-index: 5;
    }
  }
}

/* 绿色晶莹液体与波浪效果 */
.liquid {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  background: linear-gradient(180deg, #a7dbca 0%, #68b998 100%);
  transition: height 0.8s ease-in-out;

  /* 前后双层波浪纹，形成立体动态感 */
  .wave {
    position: absolute;
    top: -12px;
    width: 200%;
    height: 20px;
    background-size: 50% 100%;
  }

  .wave-front {
    left: 0;
    background: radial-gradient(circle at 50% 0%, transparent 12px, #a7dbca 13px);
    background-size: 30px 20px;
    animation: moveWave 4s linear infinite;
    opacity: 0.9;
    z-index: 4;
  }

  .wave-back {
    right: 0;
    background: radial-gradient(circle at 50% 0%, transparent 12px, #8ecfb9 13px);
    background-size: 35px 20px;
    animation: moveWave 6s linear infinite reverse;
    opacity: 0.6;
    z-index: 3;
    top: -14px;
  }
}

/* 液体内漂浮的小水滴气泡暗纹 */
.bubbles {
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  
  .bubble {
    position: absolute;
    font-size: 16px;
    opacity: 0.25;
    filter: grayscale(100%) brightness(2); /* 让Emoji呈现半透明白色水滴感 */
    
    &.b1 { left: 25%; bottom: 20%; animation: floatUp 3s infinite ease-in-out; }
    &.b2 { left: 50%; bottom: 45%; font-size: 22px; animation: floatUp 4s infinite ease-in-out 1s; }
    &.b3 { left: 70%; bottom: 15%; animation: floatUp 3.5s infinite ease-in-out 0.5s; }
  }
}

/* ================= 装饰物（小兔子与绿叶） ================= */
.mascot-rabbit {
  position: absolute;
  right: 15px;
  bottom: 12px;
  z-index: 3;
  display: flex;
  flex-direction: column;
  align-items: center;

  .rabbit-emoji {
    font-size: 56px;
    filter: drop-shadow(0 4px 6px rgba(0,0,0,0.05));
  }

  /* 纯 CSS 还原图中小兔子的绿色小围巾 */
  .scarf {
    width: 32px;
    height: 8px;
    background-color: #5b8772;
    border-radius: 4px;
    position: absolute;
    bottom: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);

    &::after {
      content: '';
      position: absolute;
      right: 4px;
      top: 6px;
      width: 6px;
      height: 12px;
      background-color: #5b8772;
      border-radius: 2px;
    }
  }
}

.bottom-leaves {
  position: absolute;
  left: 15px;
  bottom: 10px;
  font-size: 24px;
  opacity: 0.6;
  z-index: 1;
}

/* ================= 动画特效 ================= */
@keyframes moveWave {
  0% { transform: translateX(0); }
  100% { transform: translateX(-50%); }
}

@keyframes floatUp {
  0% { transform: translateY(0) scale(0.9); opacity: 0.2; }
  50% { transform: translateY(-15px) scale(1.05); opacity: 0.4; }
  100% { transform: translateY(-30px) scale(0.9); opacity: 0; }
}
</style>