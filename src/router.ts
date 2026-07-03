import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from './views/HomeView.vue'

const routes = [
  { path: '/', name: 'home', component: HomeView },
  { path: '/marking', name: 'marking', component: () => import('./views/MarkingView.vue') },
  { path: '/study', name: 'study', component: () => import('./views/StudyView.vue') }
]

export default createRouter({ history: createWebHashHistory(), routes })
