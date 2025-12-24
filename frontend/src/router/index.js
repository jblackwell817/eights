import { createRouter, createWebHistory } from 'vue-router'
import MensResults from '../views/MensResults.vue'
import WomensResults from '../views/WomensResults.vue'
import CrewResults from '../views/CrewResults.vue'

const routes = [
  { path: '/results/men', name: 'MensResults', component: MensResults },
  { path: '/results/women', name: 'WomensResults', component: WomensResults },
  {
    path: '/results/:gender/:college/:boat',
    name: 'CrewResults',
    component: CrewResults,
    props: true,
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
  linkActiveClass: 'text-blue-500 font-bold',
})

export default router
