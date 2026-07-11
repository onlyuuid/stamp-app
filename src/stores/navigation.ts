import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useNavigationStore = defineStore('navigation', () => {
  // 当前激活的菜单 key/路由路径
  const currentMenu = ref<String>('my-task');
  
  // 当前页面右侧需要展示的主标题
  const pageTitle = ref<String>('我的任务');
  
  // 当前页面的副标题（寄语）
  const pageSubtitle = ref<String>('不积跬步无以至千里，不积小流无以成江海');

  // 更新导航信息的统一方法
  const setNavigation = (key:String, title:String, subtitle = '不积跬步无以至千里，不积小流无以成江海') => {
    currentMenu.value = key;
    pageTitle.value = title;
    pageSubtitle.value = subtitle;
  };

  return {
    currentMenu,
    pageTitle,
    pageSubtitle,
    setNavigation
  };
});