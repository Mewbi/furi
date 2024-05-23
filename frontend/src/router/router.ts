import { createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
      {
        path: '/',
        name: 'home',
        component: () => import('../views/HomePage.vue'),
      },
      {
        path: '/not-found',
        name: 'not-found',
        component: () => import('../views/NotFoundPage.vue'),
      },
      {
        path: '/analytics',
        name: 'analytics',
        component: () => import('../views/AnalyticsPage.vue'),
      }
    ]
  });
