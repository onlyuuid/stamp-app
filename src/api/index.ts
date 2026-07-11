import { invoke } from "@tauri-apps/api/core"
import { message } from "ant-design-vue";

/**
 * 统一调用 Tauri Command
 * @param command Rust Command 名称
 * @param args 参数
 */
export async function request<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    message.error(msg);
    throw err;
  }
}