import { createRouter, createWebHistory } from 'vue-router'
import MensResults from '../views/MensResults.vue'
import WomensResults from '../views/WomensResults.vue'
import CrewResults from '../views/CrewResults.vue'

const routes = [
  { path: '/results/men', component: MensResults },
  { path: '/results/women', component: WomensResults },
  { path: '/results/men/:college/:boat', component: CrewResults },
  { path: '/results/women/:college/:boat', component: CrewResults },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
