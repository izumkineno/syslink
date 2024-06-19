import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from 'pinia'
import "element-plus/dist/index.css";
import './index.css'
import VxeTable from 'vxe-table'
import 'vxe-table/lib/style.css'
import VxeUI from 'vxe-pc-ui'
import 'vxe-pc-ui/lib/style.css'

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(VxeTable)
app.use(VxeUI)
app.mount('#app')