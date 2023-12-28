import 'normalize.css/normalize.css'

import { createApp } from 'vue'
import App from './App.vue'
import 'prismjs/themes/prism.css'
import VueTippy from 'vue-tippy'
import 'tippy.js/dist/tippy.css'
import Notifications from '@kyvg/vue3-notification'

const app = createApp(App)
app.use(VueTippy, {
  directive: 'tippy',
  defaultProps: {
    placement: 'right',
    allowHTML: true
  }
})

app.use(Notifications)

app.mount('#app')
