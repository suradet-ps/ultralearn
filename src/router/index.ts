import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'landing',
      component: () => import('../views/LandingView.vue'),
    },
    {
      path: '/plan/:id',
      name: 'plan',
      component: () => import('../views/PlanOverview.vue'),
    },
    {
      path: '/plan/:id/principle/:principleId',
      name: 'principle',
      component: () => import('../views/PrincipleDetail.vue'),
    },
  ],
})

export default router
