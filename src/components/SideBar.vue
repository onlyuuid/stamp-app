<template>
  <div class="sideBar-container">
    <div class="window-controls">
      <span class="control-dot close"></span>
      <span class="control-dot minimize"></span>
      <span class="control-dot maximize"></span>
    </div>

    <div class="sidebar-header">
      <div class="logo-area">
        <span class="logo-icon">💧</span>
        <h1 class="logo-title">点滴</h1>
      </div>
      <p class="logo-slogan">积小流 · 成江海</p>
    </div>

    <div class="menu-area">
      <a-menu
        v-model:selectedKeys="selectedKeys"
        mode="inline"
        :selectable="true"
        class="custom-menu"
      >
        <a-menu-item key="overview" @click="handleMenuClick('my-task','/index')">
          <template #icon>
            <span class="menu-icon-wrapper">🏠</span>
          </template>
          <span>我的任务</span>
        </a-menu-item>
        
        <a-menu-item key="statistics" @click="handleMenuClick('data-statistics','/data-statistics')">
          <template #icon>
            <span class="menu-icon-wrapper">📊</span>
          </template>
          <span>数据统计</span>
        </a-menu-item>
        
        <!-- <a-menu-item key="calendar" @click="handleMenuClick('date-view','/date-view')">
          <template #icon>
            <span class="menu-icon-wrapper">📅</span>
          </template>
          <span>日历视图</span>
        </a-menu-item> -->
        
        <!-- <a-menu-item key="settings" @click="handleMenuClick('setting','/setting')">
          <template #icon>
            <span class="menu-icon-wrapper">⚙️</span>
          </template>
          <span>设置</span>
        </a-menu-item> -->
      </a-menu>
    </div>

    <div class="sidebar-footer">
      <img 
        :src="menuIcon" 
        alt="illustration" 
        class="footer-illustration"
      />
      <div class="version-text">v1.0.0</div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import menuIcon from '../assets/image/menu-icon.png'
import { useNavigationStore } from '@/stores/navigation';

const router = useRouter()
const navStore = useNavigationStore();

const handleMenuClick = (menuKey,url) => {
  if (menuKey === 'my-task' ) {
    navStore.setNavigation('my-task', '我的任务');
  } else if (menuKey === 'data-statistics') {
    navStore.setNavigation('data-statistics', '数据统计');
  }else if (menuKey === 'date-view') {
    navStore.setNavigation('date-view', '日历视图');
  }else if (menuKey === 'setting') {
    navStore.setNavigation('setting', '设置');
  }
  handleRouter(url);
};

/** 路由跳转 */
const handleRouter = (url) => {
    router.replace(url);
}

// 当前选中的菜单项，默认选中'总览'
const selectedKeys = ref(['overview']);

</script>

<style lang="scss" scoped>
.sideBar-container {
  width: 260px;
  height: 100vh;
  background: linear-gradient(180deg, #f3f7fa 0%, #fdfefe 100%);
  display: flex;
  flex-direction: column;
  padding: 20px 16px;
  box-sizing: border-box;
  user-select: none;
  border-right: 1px solid rgba(0, 0, 0, 0.03);
//   border: 1px solid red;
  /* Mac 窗口三色点 */
  .window-controls {
    display: flex;
    gap: 8px;
    margin-bottom: 30px;
    padding-left: 4px;

    .control-dot {
      width: 12px;
      height: 12px;
      border-radius: 50%;
      display: inline-block;
      
      &.close { background-color: #ff5f56; }
      &.minimize { background-color: #ffbd2e; }
      &.maximize { background-color: #27c93f; }
    }
  }

  /* 顶部 Logo 区 */
  .sidebar-header {
    padding-left: 30px;
    margin-bottom: 40px;
    // border: 1px solid red;
    .logo-area {
      display: flex;
      align-items: center;
      gap: 12px;

      .logo-icon {
        font-size: 28px;
        filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1));
      }

      .logo-title {
        font-size: 26px;
        font-weight: 600;
        color: #1e293b;
        margin: 0;
        letter-spacing: 2px;
      }
    }

    .logo-slogan {
      margin: 6px 0 0 0;
      font-size: 13px;
      color: #94a3b8;
      letter-spacing: 1px;
      padding-left: 10px;
    }
  }

  /* 中间菜单区 */
  .menu-area {
    flex: 1;

    /* 深度定制 Ant Design Vue Menu 样式使其完美契合原型 */
    :deep(.custom-menu.ant-menu) {
      background: transparent;
      border-inline-end: none !important;

      .ant-menu-item {
        height: 50px;
        line-height: 50px;
        margin-block: 8px !important;
        border-radius: 25px; // 胶囊圆角
        padding-left: 20px !important;
        color: #64748b;
        font-size: 15px;
        font-weight: 500;
        transition: all 0.2s ease;

        // 统一图标大小与间距
        .menu-icon-wrapper {
          font-size: 18px;
          margin-right: 8px;
        }

        // 悬浮状态
        &:hover {
          color: #3b82f6;
          background-color: rgba(59, 130, 246, 0.04);
        }

        // 选中状态 (精细还原原图的淡蓝胶囊)
        &.ant-menu-item-selected {
          color: #2563eb !important;
          background-color: #e0ecfb !important; 
          font-weight: 600;

          &::after {
            display: none; // 去除 ant design 默认的右侧右边条
          }
        }
      }
    }
  }

  /* 底部插画及版本号 */
  .sidebar-footer {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    margin-top: auto; // 自动推至容器最底部

    .footer-illustration {
      width: 100px;
      height: auto;
      opacity: 0.85;
      margin-bottom: 15px;
      pointer-events: none;
      object-fit: contain;
    }

    .version-text {
      font-size: 12px;
      color: #cbd5e1;
      letter-spacing: 0.5px;
    }
  }
}
</style>