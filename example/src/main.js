import {createApp} from 'vue'
import App from './App.vue'
import 'prismjs/themes/prism.css';
import VueTippy from 'vue-tippy'
import "tippy.js/themes/light.css";

const app = createApp(App);
app.use(VueTippy,{
  directive: 'tippy',
  position: 'right',
  theme: 'light',
  size: 'small',
  allowHTML: true,
});
app.mount("#app");
