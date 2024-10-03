import "primeicons/primeicons.css";
import "../public/main.css";

import { createApp } from "vue";
import PrimeVue from "primevue/config";
import { definePreset } from '@primevue/themes';
import Lara from "@primevue/themes/lara";


import DataTable from 'primevue/datatable';
import FileUpload from "primevue/fileupload";
import Card from 'primevue/card'
import Select from 'primevue/Select';
import Button from 'primevue/button';

import App from "./App.vue";

const CustomColors = definePreset(Lara, {
  semantic: {
    primary: {
      50: "{violet.50}",
      100: "{violet.100}",
      200: "{violet.200}",
      300: "{violet.300}",
      400: "{violet.400}",
      500: "{violet.500}",
      600: "{violet.600}",
      700: "{violet.700}",
      800: "{violet.800}",
      900: "{violet.900}",
      950: "{violet.950}",
    },
  },
});

const app = createApp(App);

app.use(PrimeVue, {
  theme: {
    preset: CustomColors,
  },
});

app.component("Button", Button);
app.component("FileUpload", FileUpload);
app.component('Card', Card);
app.component('Select', Select);
app.component('DataTable', DataTable);

app.mount("#app");
