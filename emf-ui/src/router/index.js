import { createRouter, createWebHistory } from 'vue-router'

import MapView from '@/views/MapView.vue'
import EntitiesView from '@/views/EntitiesView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: { name: 'map' },
    },
    {
      path: '/map/',
      name: 'map',
      component: MapView,
    },
    {
      path: '/map/:location',
      name: 'map-precise-location',
      component: MapView,
    },
    {
      path: '/entities/',
      name: 'entities',
      component: EntitiesView,
    },
  ],
})

export default router
