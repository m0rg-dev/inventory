<script setup>
import dayjs from "dayjs";
import localizedFormat from "dayjs/plugin/localizedFormat";

import * as ItemAPI from "./ItemAPI.js";
import BarcodeScanner from "./BarcodeScanner.vue";

dayjs.extend(localizedFormat);
</script>


<script>
export default {
  components: { BarcodeScanner },
  data: () => ({
    items: null,
    loadingState: "pre-fetch",
    itemsLoaded: 0,
    itemsRemaining: 0,
    error: null,
  }),

  async created() {
    await this.fetchItems();
  },

  methods: {
    async fetchItems() {
      try {
        let ids = await (await fetch("/items/")).json();
        this.loadingState = "fetch-list";

        this.items = {};
        this.itemsRemaining = ids.length;
        for (const id of ids) {
          this.items[id] = await (await fetch(`/items/${id}`)).json();
          this.itemsLoaded++;
        }

        this.loadingState = "fetched";
      } catch (e) {
        console.log(e);
        this.error = e;
      }
    },

    async checkOut(id) {
      this.items[id]._fe_await = true;
      this.items[id] = await ItemAPI.checkOut(id);
    },

    async checkIn(id) {
      this.items[id]._fe_await = true;
      this.items[id] = await ItemAPI.checkIn(id);
    },

    async onScan(t) {
      this.$router.push(`/items/${t.toLowerCase()}`);
    },
  },
};
</script>

<template>
  <div class="container">
    <div
      class="d-flex justify-content-center m-3"
      v-if="loadingState != 'fetched'"
    >
      <button class="btn btn-primary" disabled v-if="!error">
        <span
          class="spinner-border spinner-border-sm"
          role="status"
          aria-hidden="true"
          v-if="loadingState == 'fetch-list'"
        ></span>
        Loading...
      </button>

      <button class="btn btn-danger" disabled v-if="error">
        Failed to load items.
      </button>
    </div>

    <div class="progress" v-if="loadingState == 'fetch-list'">
      <div
        class="progress-bar"
        :style="{ width: (itemsLoaded * 100) / itemsRemaining + '%' }"
      ></div>
    </div>

    <div class="container">
      <div class="alert alert-danger" role="alert" v-if="error">
        {{ error }}
        <pre>{{ error.stack }}</pre>
      </div>
    </div>

    <table class="table" v-if="items">
      <thead>
        <th style="width: 7rem">Container?</th>
        <th>Description</th>
        <th>Stored In</th>
        <th style="width: 20rem">Checked Out</th>
      </thead>
      <tbody>
        <tr v-for="(item, id) in items" :key="id">
          <td>{{ item.is_container }}</td>
          <td>
            <router-link :to="'/items/' + id">{{
              item.description
            }}</router-link>
          </td>
          <td>{{ item.parent_container }}</td>
          <td>
            <button
              class="btn btn-sm btn-success"
              v-if="!item.checked_out"
              @click="checkOut(id)"
            >
              <span
                class="spinner-border spinner-border-sm"
                role="status"
                v-if="item._fe_await"
              ></span>
              Checked In
            </button>

            <button
              class="btn btn-sm btn-outline-primary"
              v-if="item.checked_out"
              @click="checkIn(id)"
            >
              <span
                class="spinner-border spinner-border-sm"
                role="status"
                v-if="item._fe_await"
              ></span>
              {{ dayjs(item.checked_out).format("llll") }}
            </button>
          </td>
        </tr>
      </tbody>
    </table>

    <barcode-scanner @result="onScan" />
  </div>
</template>
