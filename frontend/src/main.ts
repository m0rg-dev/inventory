import { createApp } from "vue";
import BootstrapVue3 from 'bootstrap-vue-3'
import * as VueRouter from "vue-router";
import ItemList from "./components/ItemList.vue";
import SingleItem from "./components/SingleItem.vue";

import 'bootstrap/dist/css/bootstrap.css';
import 'bootstrap-vue-3/dist/bootstrap-vue-3.css';

const routes = [
    { path: "/", component: ItemList },
    { path: "/items/:id", component: SingleItem }
];

const router = VueRouter.createRouter({
    history: VueRouter.createWebHashHistory(),
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    routes: routes as any
});


const app = createApp({});
app.use(router);
app.use(BootstrapVue3);
app.mount("#app");
