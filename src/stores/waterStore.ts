import { defineStore } from 'pinia';
import { ref } from 'vue';
import { listTask } from "../api/task";
import type { Task } from "../types/task";
import { getLogByDate } from '../api/water_log'

export const useWaterStore = defineStore('water', () => {

    const taskList = ref<Task[]>([]);
    const todayTotalWater = ref(0);

    const getLogList = async () => {
        const today = new Date();
        const year = today.getFullYear();
        const month = String(today.getMonth() + 1).padStart(2, '0');
        const day = String(today.getDate()).padStart(2, '0');
        const dateStr = `${year}-${month}-${day}`;
        let logs = await getLogByDate(dateStr);
        let dayInvest = logs.reduce((sum,item) => {
            return sum + (item.waterNum || 0);   
        },0)

        return dayInvest
    }

    // 动态更新今日水滴的逻辑
    const refreshTodayWater = async () => {

        todayTotalWater.value = await getLogList();
    }

    return { todayTotalWater, refreshTodayWater };
});