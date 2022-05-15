import {createApp} from "vue";
import * as VueRouter from "vue-router";
import ItemList from "./components/ItemList.vue";
import SingleItem from "./components/SingleItem.vue";

import 'bootstrap/dist/css/bootstrap.css';
import 'bootstrap-vue/dist/bootstrap-vue.css';

const routes = [
    {path: "/", component: ItemList},
    {path: "/items/:id", component: SingleItem}
];

const router = VueRouter.createRouter({
    history: VueRouter.createWebHashHistory(),
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    routes: routes as any
});


const app = createApp({});
app.use(router);
app.mount("#app");
