import { request } from "./index";
import type { WaterLog,MapData } from "../types/water_log";


/**
 * 查询日志列表
 */
export function listLog(): Promise<WaterLog[]> {
  return request<WaterLog[]>("list_water_log");
}

/**
 * 根据ID查询日志
 */
export function getLogById(logId: number): Promise<WaterLog> {
  return request<WaterLog>("find_by_log_id", {
    logId,
  });
}

/**
 * 根据任务ID查询日志
 */
export function getLogByTaskId(taskId: number): Promise<[]> {
  return request<[]>("find_by_task_id", {
    taskId,
  });
}

/**
 * 根据日期查询日志
 */
export function getLogByDate(date:String): Promise<WaterLog[]> {
   return request<WaterLog[]>("find_by_date", {
    date,
  });
}

/**
 * 新增日志
 */
export function createLog(log: WaterLog): Promise<WaterLog> {
  return request<WaterLog>("create_water_log", {
    log,
  });
}

/**
 * 修改日志
 */
export function updateLog(log: WaterLog): Promise<void> {
  return request<void>("update_water_log", {
    log,
  });
}

/**
 * 删除日志
 */
export function deleteLog(taskId: number): Promise<number> {
  return request<number>("delete_water_log", {
    taskId,
  });
}

// 数据统计
/**
 * 本周投入
 */
export function week_total(): Promise<number> {
  return request<number>("week_total");
}

/**
 * 连续坚持天数 streak_days
 */
export function streak_days(): Promise<number> {
  return request<number>("streak_days");
}


/**
 * 热力图
 */
export function get_heatmap(year:number): Promise<MapData[]> {
  return request<MapData[]>("get_heatmap",{
    year
  });
}

