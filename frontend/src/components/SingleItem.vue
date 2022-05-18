<script setup lang="ts">
import dayjs from "dayjs";
import localizedFormat from "dayjs/plugin/localizedFormat";

import Item from "./ItemAPI";
import Label from "./Label.vue";
import ScannerModal from "./ScannerModal.vue";

dayjs.extend(localizedFormat);
</script>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  data() {
    return {
      item: null as Item,
      editable_tag: null,
      edit_key_to: null,
      editable_description: false,
      edit_description_to: null,
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
    async renameTag(k1: string, k2: string, v: string) {
      await this.item.deleteTag(k1);
      await this.item.updateTag(k2, v);
      this.editable_tag = null;
    },

    editTag(k: string) {
      this.editable_tag = this.edit_key_to = k;
      this.editable_description = false;
    },

    newTag() {
      this.item.tags["New Tag"] = "New Value";
      this.editable_tag = this.edit_key_to = "New Tag";
    },

    editDescription() {
      this.editable_tag = null;
      this.editable_description = true;
      this.edit_description_to = this.item.getDescription();
    },

    async updateDescription() {
      await this.item.setDescription(this.edit_description_to);
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
        <input type="text" v-model="edit_description_to" @blur="updateDescription()" v-if="editable_description" />
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
            <li class="list-group-item">
              <button class="btn btn-sm btn-danger mr-3" @click="item.deleteTag(k)">
                <i class="bi-trash"></i></button>&nbsp;
              <span v-if="k == editable_tag">
                <input type="text" v-model="edit_key_to" @blur="renameTag(k, edit_key_to, item.tags[k])" />:
                <input type="text" v-model="item.tags[k]" @blur="renameTag(k, edit_key_to, item.tags[k])" />
              </span>
              <span v-else @click="editTag(k)">
                <code>{{ k }}: {{ item.tags[k] }}</code>
              </span>
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