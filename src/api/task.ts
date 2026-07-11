import { request } from "./index";
import type { Task } from "../types/task";
import type {TaskWeeklyData} from "../types/weekly_query"


/**
 * 查询任务列表
 */
export function listTask(): Promise<Task[]> {
  return request<Task[]>("list_task");
}

/**
 * 根据ID查询
 */
export function getTaskById(taskId: number): Promise<Task> {
  return request<Task>("find_by_id", {
    taskId,
  });
}

/**
 * 新增任务
 */
export function createTask(task: Task): Promise<Task> {
  return request<Task>("create_task", {
    task,
  });
}

/**
 * 修改任务
 */
export function updateTask(task: Task): Promise<Task> {
  return request<Task>("update_task", {
    task,
  });
}

/**
 * 删除任务
 */
export function deleteTask(taskId: number): Promise<number> {
  return request<number>("delete_task", {
    taskId,
  });
}

/**
 * 本周数据统计
 */
export function weeklyStats(date: String): Promise<TaskWeeklyData> {
  return request<TaskWeeklyData>("weekly_stats", {
    date,
  });
}
