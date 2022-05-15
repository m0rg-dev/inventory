<script setup>
import dayjs from "dayjs";
import localizedFormat from "dayjs/plugin/localizedFormat";

import * as ItemAPI from "./ItemAPI.js";

dayjs.extend(localizedFormat);
</script>

<script>
export default {
  data: () => ({ item: null, editable_tag: null, edit_key_to: null }),

  async created() {
    await this.fetchItem(this.$route.params.id);
  },

  methods: {
    async fetchItem(id) {
      this.item = await ItemAPI.fetchItem(id);
    },

    async checkOut(id) {
      this.item = await ItemAPI.checkOut(id);
    },

    async checkIn(id) {
      this.item = await ItemAPI.checkIn(id);
    },

    async updateTag(id, k, v) {
      this.item = await ItemAPI.updateTag(id, k, v);
      this.editable_tag = null;
    },

    async renameTag(id, k1, k2, v) {
      this.item = await ItemAPI.deleteTag(id, k1);
      this.item = await ItemAPI.updateTag(id, k2, v);
      this.editable_tag = null;
    },

    async deleteTag(id, k) {
      this.item = await ItemAPI.deleteTag(id, k);
    },

    newTag() {
      this.item.tags["New Tag"] = "New Value";
      this.editable_tag = this.edit_key_to = "New Tag";
    },
  },
};
</script>

<template>
  <div class="container p-3">
    <router-link to="/" class="btn btn-primary mb-3">Back</router-link>
    <div class="card p-3" v-if="item">
      <div class="card-body">
        <h4 class="card-title">{{ item.description }}</h4>
        <ul class="list-group list-group-flush">
          <li class="list-group-item">
            ID: <code>{{ item.id }}</code>
          </li>
          <li class="list-group-item" v-if="item.parent_container">
            Contained in: <code>{{ item.parent_container }}</code>
          </li>
          <li class="list-group-item" v-if="item.checked_out">
            Checked out at: {{ dayjs(item.checked_out).format("llll") }}
            <button
              class="btn btn-sm btn-primary ml-5"
              @click="checkIn(item.id)"
            >
              <i class="bi-download"></i>
              Check in
            </button>
          </li>
          <li class="list-group-item" v-else>
            Not checked out.
            <button class="btn btn-sm btn-success" @click="checkOut(item.id)">
              <i class="bi-upload"></i>
              Check out
            </button>
          </li>
          <li class="list-group-item" v-if="item.destroyed">
            Destroyed at: {{ dayjs(item.destroyed).format("llll") }}
          </li>
          <template v-for="(v, k) in item.tags" :key="k">
            <li class="list-group-item">
              <button class="btn btn-sm btn-danger mr-3">
                <i class="bi-trash"></i></button
              >&nbsp;
              <span v-if="k == editable_tag">
                <input
                  type="text"
                  v-model="edit_key_to"
                  @keyup.enter="
                    renameTag(item.id, k, edit_key_to, item.tags[k])
                  "
                />:
                <input
                  type="text"
                  v-model="item.tags[k]"
                  @keyup.enter="renameTag(item.id, k, edit_key_to, v)"
                />
              </span>
              <span v-else @click="this.editable_tag = this.edit_key_to = k">
                <code>{{ k }}: {{ v }}</code>
              </span>
            </li>
          </template>
        </ul>
        <button class="btn btn-sm btn-success" @click="newTag()">
          <i class="bi-plus-lg"></i> Add tag
        </button>
      </div>
    </div>
  </div>
</template>