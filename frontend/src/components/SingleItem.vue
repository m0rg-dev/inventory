<script setup lang="ts">
import dayjs from "dayjs";
import localizedFormat from "dayjs/plugin/localizedFormat";

import Item from "./ItemAPI";
import Label from "./Label.vue";
import ScannerModal from "./ScannerModal.vue";

dayjs.extend(localizedFormat);
</script>

<script lang="ts">
import { defineComponent, nextTick } from "vue";

export default defineComponent({
  data() {
    return {
      item: null as Item,
      editable_key: null,
      editable_value: null,
      edit_to: null,
      editable_description: false,
      confirm_delete: false,
      associating: false,
      parent_description: "",
    };
  },

  async created() {
    this.item = await Item.load(this.$route.params.id);
    this.getParentDescription();
  },

  methods: {
    editKey(k: string) {
      this.editable_key = this.edit_to = k;
      this.editable_description = false;
    },

    async doneEditingKey(e: FocusEvent, k: string) {

      const v = this.item.tags[k];
      await this.item.deleteTag(k);
      await this.item.updateTag(this.edit_to, v);
      this.editable_key = null;

      nextTick(() => {
        if (e.relatedTarget && e.relatedTarget instanceof Element && e.relatedTarget.id) {
          document.getElementById(e.relatedTarget.id)?.focus();
        }
      });
    },

    editValue(k: string) {
      this.editable_value = k;
      this.edit_to = this.item.tags[k];
      this.editable_description = false;
    },

    async doneEditingValue(e: FocusEvent, k: string) {

      await this.item.updateTag(k, this.edit_to);
      this.editable_value = null;

      nextTick(() => {
        if (e.relatedTarget && e.relatedTarget instanceof Element && e.relatedTarget.id) {
          document.getElementById(e.relatedTarget.id)?.focus();
        }
      });
    },

    newTag() {
      this.item.tags["New Tag"] = "New Value";
      nextTick(() => {
        document.getElementById("key-New Tag").focus();
      });
    },

    editDescription() {
      this.editable_key = null;
      this.editable_description = true;
      this.edit_to = this.item.getDescription();
    },

    async updateDescription() {
      await this.item.setDescription(this.edit_to);
      this.editable_description = false;
    },

    confirmDeletion() {
      this.confirm_delete = true;
    },

    cancelDeletion() {
      this.confirm_delete = false;
    },

    async doDelete() {
      await this.item.delete();
      this.$router.push("/");
    },

    startAssociating() {
      this.associating = true;
    },

    stopAssociating() {
      this.associating = false;
    },

    async associate(id: string) {
      await this.item.setParent(id);
      this.associating = false;
      this.getParentDescription();
    },

    async getParentDescription() {
      if (this.item?.getParent()) {
        this.parent_description = (await Item.load(this.item.getParent())).getDescription()
      } else {
        return "";
      }
    }
  },
});
</script>

<template>
  <div class="container p-3">
    <router-link to="/" class="btn btn-primary mb-3"><i class="bi-arrow-left"></i> Back</router-link>
    <div class="card p-3" v-if="item">
      <div class="card-body">
        <input type="text" v-model="edit_to" @blur="updateDescription()" v-if="editable_description" />
        <h4 class="card-title" v-else @click="editDescription()">
          {{ item.getDescription() || "<NO DESCRIPTION>" }}
        </h4>
        <ul class="list-group list-group-flush">
          <li class="list-group-item">
            ID: <code>{{ item.getID() }}</code>
          </li>



          <li class="list-group-item" v-if="item.getParent()">
            Contained in: {{ parent_description }}
            <button class="btn btn-sm btn-secondary" @click="item.removeParent()"><i class="bi-box-arrow-up"></i>
              Remove</button>
          </li>
          <li class="list-group-item" v-else>Not inside a container.
            <button class="btn btn-sm btn-primary" @click="startAssociating()"><i class="bi-archive"></i>
              Associate</button>
          </li>



          <li class="list-group-item" v-if="item.checkedOutAt()">
            Checked out at: {{ dayjs(item.checkedOutAt()).format("llll") }}
            <button class="btn btn-sm btn-primary ml-5" @click="item.checkIn()">
              <i class="bi-download"></i>
              Check in
            </button>
          </li>
          <li class="list-group-item" v-else>
            Not checked out.
            <button class="btn btn-sm btn-success" @click="item.checkOut()">
              <i class="bi-upload"></i>
              Check out
            </button>
          </li>



          <template v-for="k of Object.keys(item.tags).sort()" :key="k">
            <li class="list-group-item" @blur="editable_key = null" tabindex=0>
              <button class="btn btn-sm btn-danger me-3" @click="item.deleteTag(k)">
                <i class="bi-trash"></i></button>
              <span v-if="k == editable_key"><input type="text" :id="'key-' + k" v-model="edit_to"
                  @blur="doneEditingKey($event, k)" /></span>
              <span v-else><code @focus="editKey(k)" tabindex=0 :id="'key-' + k">{{ k }}</code></span><code>:</code>
              <span v-if="k == editable_value"><input type="text" :id="'value-' + k" v-model="edit_to"
                  @blur="doneEditingValue($event, k)" /></span>
              <span v-else><code @focus="editValue(k)" tabindex=0 :id="'value-' + k">{{ item.tags[k] }}</code></span>
            </li>
          </template>
        </ul>
        <button class="btn btn-sm btn-success" @click="newTag()">
          <i class="bi-plus-lg"></i> Add tag
        </button>
        <Label :id="item.getID()" :desc="item.getDescription()" class="mt-3"></Label>

        <button class="btn btn-outline-danger mt-3" @click="confirmDeletion()" v-if="!confirm_delete"><i
            class="bi-trash"></i>Delete {{
                item.getDescription()
            }} </button>
        <div class="d-flex mt-3" v-if="confirm_delete">
          <div>
            <button class="btn btn-secondary" @click="cancelDeletion()"><i class="bi-cross"></i>Cancel
              Deletion</button>
          </div>
          <div style="margin-left: auto">
            <button class="btn btn-danger" @click="doDelete()">Confirm: Delete {{
                item.getDescription()
            }} </button>
          </div>
        </div>
      </div>
    </div>

    <Teleport to="body">
      <ScannerModal :show="associating" :interactive="false" @close="stopAssociating" @scan="associate">
      </ScannerModal>
    </Teleport>
  </div>
</template>