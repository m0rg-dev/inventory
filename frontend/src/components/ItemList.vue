<script setup>
import dayjs from "dayjs";
import { v4 as uuidv4 } from "uuid";
import localizedFormat from "dayjs/plugin/localizedFormat";

import Item, * as ItemAPI from "./ItemAPI";
import ScannerModal from "./ScannerModal.vue";

dayjs.extend(localizedFormat);
</script>


<script>
export default {
  data: () => ({
    items: null,
    loadingState: "pre-fetch",
    itemsLoaded: 0,
    itemsRemaining: 0,
    error: null,
    scanning: false,
  }),

  async created() {
    await this.fetchItems();
  },

  methods: {
    async fetchItems() {
      try {
        let loaded = await (await fetch("/api/items/")).json();
        console.log(loaded);
        this.loadingState = "fetch-list";

        this.items = {};

        for (const id in loaded) {
          this.items[id] = new Item(id, loaded[id].tags);
        }

        for (const id in this.items) {
          const pid = this.items[id].getParent();
          if (pid) {
            this.items[id]._parent_desc = this.items[pid].getDescription();
          }
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

    <table class="table" v-if="items">
      <thead>
        <th>Description</th>
        <th>Stored In</th>
        <th style="width: 20rem">Checked Out</th>
      </thead>
      <tbody>
        <tr v-for="(item, id) in items" :key="id">
          <td>
            <router-link :to="'/items/' + id">{{
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
