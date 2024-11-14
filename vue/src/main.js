import { createApp } from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import { loadFonts } from './plugins/webfontloader'
import { createWebHistory, createRouter } from 'vue-router'
import Wordnet from './components/Wordnet.vue'
import editor from './components/editor.vue'
import './assets/main.css'
import './assets/fonts.css'

loadFonts()

const routes = [
    { path: "/:index/:query", component: Wordnet },
    { path: '/', component: Wordnet },
    { path: '/edit', component: editor },
    //{ path: '/:index/:query', component: Wordnet, props: true }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

createApp(App)
  .use(vuetify)
  .use(router)
  .mount('#app')
