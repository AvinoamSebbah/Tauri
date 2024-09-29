import { createApp } from 'vue';
import App from './App.vue';
import router from './router/index';

import Aura from '@primevue/themes/aura';
import { definePreset } from '@primevue/themes';
import AuraPreset from './presets/aura/index';
import PrimeVue from 'primevue/config';
import ConfirmationService from 'primevue/confirmationservice';
import ToastService from 'primevue/toastservice';
import { createI18n } from 'vue-i18n';
import '@/assets/styles.scss';
import '@/assets/tailwind.css';

const app = createApp(App);

const i18n = createI18n({
    locale: 'he',
    fallbackLocale: 'he',
    globalInjection: true,
  });

app.use(router);
const MyPreset = definePreset(Aura, AuraPreset);
app.use(PrimeVue, {
     theme: {
         preset: MyPreset,
         options: {
             darkModeSelector: '.app-dark'
         }
     }
 });
app.use(ToastService);
app.use(ConfirmationService);
app.use(i18n);

app.mount('#app');
