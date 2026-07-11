import { createWebHashHistory, createRouter } from 'vue-router';

// 公共路由
export const routes = [
    {
        path: '/',
        redirect: 'index'
    },
    {
        path: '/index',
        component: () => import('../views/index.vue'),
        meta: {}
    },
    {
        path: '/data-statistics',
        component: () => import('../views/data-statistics/index.vue'),
        meta: {}
    },
    {
        path: '/date-view',
        component: () => import('../views/date-view/index.vue'),
        meta: {}
    },
     {
        path: '/setting',
        component: () => import('../views/sys-setting/index.vue'),
        meta: {}
    },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});

export default router;



