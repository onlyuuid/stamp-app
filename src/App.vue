<template>
  <div class="app-container">
    <aside>
      <SideBar></SideBar>
    </aside>
    <main>
      <Header></Header>
      <div class="views-container">
         <a-config-provider :locale="zhCN">
            <router-view/>
        </a-config-provider>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import SideBar from './components/SideBar.vue'
import Header from './components/Header.vue';
import zhCN from "ant-design-vue/es/locale/zh_CN";
import dayjs from "dayjs";
import "dayjs/locale/zh-cn";
dayjs.locale("zh-cn");

</script>

<style lang="scss" scoped>
/* 基础容器布局 */
.app-container{
    display:flex;
    height:100vh;
    background:#f7f9fc;
    min-width: 800px;
    overflow-x: hidden;
    .views-container{
      padding: 15px;
      height: calc(100vh - 120px);
      // border: 1px solid red;
      overflow-y:auto;
      overflow-x: hidden;
    }
    .views-container::-webkit-scrollbar{
      background-color: #f7f9fc;
      width: 10px;
    }
    .views-container::-webkit-scrollbar-thumb{
      background-color: #dcf0ff;
      border-radius: 10px;
    }
    
}

</style>