<script setup>
import dayjs from "dayjs";
import { v4 as uuidv4 } from "uuid";
import localizedFormat from "dayjs/plugin/localizedFormat";
import Fuse from "fuse.js";

import Item, * as ItemAPI from "./ItemAPI";
import ScannerModal from "./ScannerModal.vue";

dayjs.extend(localizedFormat);
</script>


<script>
export default {
  data: () => ({
    items: null,
    filtered_items: null,
    fuse: null,
    loadingState: "pre-fetch",
    itemsLoaded: 0,
    itemsRemaining: 0,
    error: null,
    scanning: false,
    search_query: null
  }),

  async created() {
    await this.fetchItems();
  },

  methods: {
    async fetchItems() {
      try {
        this.loadingState = "fetch-list";

        let items = await Item.fetchAll();

        for (const id in items) {
          const pid = items[id].getParent();
          if (pid) {
            items[id]._parent_desc = items[pid].getDescription();
          }
        }

        this.items = Object.values(items);

        this.fuse = new Fuse(Object.values(this.items), {
          includeScore: true,
          useExtendedSearch: true,
          keys: ["tags._description"]
        });

        this.loadingState = "fetched";
      } catch (e) {
        console.log(e);
        this.error = e;
      }
    },

    updateSearch() {
      if (this.search_query.length > 0) {
        this.filtered_items = this.fuse.search(this.search_query).map((i) => i.item);
        console.log(this.filtered_items);
      } else {
        this.filtered_items = null;
      }
    },

    async onScan(t) {
      this.$router.push(`/items/${t.toLowerCase()}`);
    },

    async newItem() {
      const created = new Item(uuidv4(), {});
      await created.save();

      this.$router.push(`/items/${created.getID()}`);
    },

    startScanning() {
      this.scanning = true;
    },

    stopScanning() {
      this.scanning = false;
      this.fetchItems();
    }
  },
};
</script>

<template>
  <div class="container p-3">
    <div class="d-flex justify-content-center m-3" v-if="loadingState != 'fetched'">
      <button class="btn btn-primary" disabled v-if="!error">
        <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"
          v-if="loadingState == 'fetch-list'"></span>
        Loading...
      </button>

      <button class="btn btn-danger" disabled v-if="error">
        Failed to load items.
      </button>
    </div>

    <div class="container">
      <div class="alert alert-danger" role="alert" v-if="error">
        {{ error }}
        <pre>{{ error.stack }}</pre>
      </div>
    </div>

    <div v-if="items">
      <span>
        <button @click="newItem" class="btn btn-success"><i class="bi-plus-lg"></i> Create</button>
      </span>

      <span class="ms-3">
        <button @click="startScanning" class="btn btn-primary"><i class="bi-qr-code-scan"></i> Scan</button>
      </span>
    </div>

    <input type="text" class="form-control my-3" placeholder="Search" @input="updateSearch" v-model="search_query" />

    <table class="table" v-if="items || filtered_items">
      <thead>
        <th>Description</th>
        <th>Stored In</th>
        <th style="width: 20rem">Checked Out</th>
      </thead>
      <tbody>
        <tr v-for="item of filtered_items || items" :key="item.id">
          <td>
            <router-link :to="'/items/' + item.id">{{
                          item.getDescription() || "<NO-DESCRIPTION>"
            }}</router-link>
          </td>
          <td>{{ item._parent_desc }}</td>
          <td>
            <button class="btn btn-sm btn-success" v-if="!item.checkedOutAt()" @click="item.checkOut()">
              <span class="spinner-border spinner-border-sm" role="status" v-if="item._fe_await"></span>
              Checked In
            </button>

            <button class="btn btn-sm btn-outline-primary" v-else @click="item.checkIn()">
              <span class="spinner-border spinner-border-sm" role="status" v-if="item._fe_await"></span>
              {{ dayjs(item.checkedOutAt()).format("llll") }}
            </button>
          </td>
        </tr>
      </tbody>
    </table>

    <Teleport to="body">
      <ScannerModal :show="scanning" :interactive="true" @close="stopScanning" @edit="onScan">
      </ScannerModal>
    </Teleport>
  </div>
</template>
