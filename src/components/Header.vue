<template>
    <div class="header-container">
        <div>
            <span class="navigate-title">{{ navStore.pageTitle }}</span>
            <span class="desc">{{ navStore.pageSubtitle }}</span>
        </div>
        <div>
            <div class="date">📅 {{ getToDay() }}</div>
            <div class="hydrops">今天投入 <span> {{ waterStore.todayTotalWater }} </span> 滴水</div>
        </div>
        <div class="profile">
            <a-avatar :size="55" :src="profile" />
        </div>
    </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue';
import profile from '../assets/image/profile.png'
import { useNavigationStore } from '../stores/navigation';
import { useWaterStore} from '../stores/waterStore'

const navStore = useNavigationStore();
const waterStore = useWaterStore();

onMounted(() => {
    waterStore.refreshTodayWater();
})


/** 获取今天日期 */
/** 获取今天日期及星期 */
const getToDay = () => {
  const now = new Date();
  
  let year = now.getFullYear();
  // 月份从 0 开始，所以需要 +1
  let month = now.getMonth() + 1; 
  let day = now.getDate();
  
  // 1. 🌟 使用 getDay() 获取星期数字 (0-6)
  let weekDayNum = now.getDay(); 
  
  // 2. 建立数字与中文星期的映射数组
  const weekDaysShort = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'];
  let weekDayStr = weekDaysShort[weekDayNum];

  // 3. 拼接返回最终格式：2026年7月7日 星期一
  return `${year}年${month}月${day}日 ${weekDayStr}`;
};

</script>
<style lang="scss" scoped>
.header-container{
    display: flex;
    width: calc(100vw - 260px);
    height: 120px;
    display: grid;
    grid-template-columns: 6fr 2fr 1fr;
    padding-top: 20px;
    /* 核心：禁止文本被选中 */
    user-select: none; 
    /* 兼容旧版本浏览器的写法（可选） */
    -webkit-user-select: none; /* Safari */
    -moz-user-select: none;    /* Firefox */
    -ms-user-select: none;     /* IE10+ */
    & > :nth-child(1) {
        display: grid;
        grid-template-rows: 1fr 1fr;
        line-height: 50px;
        // padding-top: 20px;
        .navigate-title{
            font-size: 30px;
            padding: 0 50px;
        }
        .desc{
            padding: 0 50px;
            color: #9fd9ff;
            
        }
    }
    & > :nth-child(2) {
        // background-color: rgb(73, 175, 55);
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
        .date{
            line-height: 50px;
        }
        .hydrops{
            line-height: 35px;
            width: 150px;
            border-radius: 20px;
            height: 40px;
            border: 1px solid #d6d6d6;
            span{
                color: #14e2c6;
                font-size: 1.2rem;
            }
        }

    }
    & > :nth-child(3) {
        display: flex;
        justify-content: center;
        align-items: center;
    }

}
</style>