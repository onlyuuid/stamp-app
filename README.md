# 💧 点滴打卡 (StampApp) — 积小流，成江海

[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.x-emerald?logo=vue.js)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**点滴打卡 (StampApp)** 是一款基于 **Tauri 2.0 + Vue 3 + Rust (SeaORM)** 构建的单机硬核、视觉治愈系习惯养成与精力全追踪桌面应用。我们拒绝焦虑和死板的打卡表格，采用“玻璃蓄水瓶”与“小兔子”的陪伴式视觉隐喻，帮助你在开发、学习与日常生活中，将时间的投入具象化为每一滴清澈的水滴，积小流以成江海。

> 项目持续更新中...
---

## 🎨 匠心独运的视觉呈现

### 1. 核心工作台：我的任务

陪伴式治愈交互。每一次专注或习惯达成，为瓶子注入清凉的水滴，小兔子会见证你走过的每一步。
![stamp-app/main/我的任务.png at main · onlyuuid/stamp-app](https://github.com/onlyuuid/stamp-app/blob/main/main/我的任务.png)

### 2. 深度复盘：多维数据统计中心

内置近一周水流成长轨迹曲线、动态全任务精力水源分配环形图、以及年度能量蓄水热力墙，全方位深度透视你的精力分配。
![stamp-app/main/我的任务.png at main · onlyuuid/stamp-app](https://github.com/onlyuuid/stamp-app/blob/main/main/数据统计.png)

---

## ✨ 核心特性

- 🦀 **现代纯内生架构**：由 **Rust 驱动底层核心**，极致内存安全，软件秒开，运行近乎零资源占用。
- 📱 **单机极速就绪**：内置全自动热迁移的 **SQLite 本地私密数据库**（通过 SeaORM 强力驱动），数据全在本地，绝不上云，绝对隐私安全。
- 🎨 **治愈交互动效**：精心调校的温润全局 UI 控件，配合跨组件 Mitt 全局响应状态总线，实现跨卡片水滴注入瞬间实时顶部动态同步。
- 📈 **深度全维度流数据**：
  - 基于高鲁棒性防断档连续打卡（Streak）追踪算法。
  - 精确到分钟级的“半小时等于一滴水”的精力换算模型。
  - 按天智能分组聚合（Group By）的高性能流水日志架构设计。
- 📦 **精巧跨平台打包**：基于高效紧凑的 Windows NSIS 编译器，安装包全中文引导，体积纤细，无缝安全下沉部署。

---

## 🏗️ 核心技术栈

### 前端 (Frontend)

- **框架**：Vue 3 (Script Setup 组合式 API)
- **路由管理**：Vue Router
- **跨组件总线**：Mitt (轻量级高灵敏全场事件广播)
- **图表驱动**：ECharts / Chart.js (高效流式响应式渲染)

### 后端 (Backend - Tauri IPC)

- **核心壳体**：Tauri 2.0 (提供底层窗口生命周期控制、安全线程隔离、本地文件系统访问)
- **异步运行时**：Tokio (驱动高性能异步上下文调度)
- **ORM 数据库层**：SeaORM (支持全自动表结构智能检测并自动执行 `CREATE TABLE IF NOT EXISTS` 保证生产环境安全落地)
- **驱动引擎**：SQLCipher / SQLite3

---

## 🛠️ 本地开发与构建指引

若要在本地编译或参与贡献本项目，请确保你的系统已正确配置好 [Rust 编译环境](https://www.rust-lang.org/learn/get-started) 和 Node.js 环境。

### 1. 克隆项目仓库

```bash
git clone https://github.com/onlyuuid/stamp-app.git
cd 你的仓库名
```
## 安装前端依赖
```bash
npm install
```
## 启动本地开发热重载 (Tauri Dev 模式)
```bash
npm run tauri dev
```
## 编译生产全中文分发包 (Production Build)
```bash
npm run tauri build
```
