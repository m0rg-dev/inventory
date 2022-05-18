<script setup lang="ts">
import BarcodeScanner from "./BarcodeScanner.vue";
</script>

<script lang="ts">
import { defineComponent } from "vue";
import Item from "./ItemAPI";

export default defineComponent({
    props: {
        show: Boolean,
        interactive: Boolean
    },

    data: () => ({
        id: "",
        show_inter: false,
        item: null
    }),

    methods: {
        close() {
            this.$emit("close");
        },

        edit() {
            this.$emit("edit", this.id);
        },

        async scan(id: string) {
            this.id = id;
            this.$emit("scan", id);
            if (this.interactive) {
                this.show_inter = true;
                try {
                    this.item = await Item.load(id);
                } catch (e) {
                    // TODO
                }
            }
        }
    }
});
</script>

<template>
    <Transition name="modal">
        <div v-if="show" class="modal-mask" @keyup.esc="close">
            <div class="card m-auto p-3" style="max-width:35rem">
                <div class="card-body">
                    <BarcodeScanner @result="scan"></BarcodeScanner>

                    <div v-if="show_inter" class="py-2">
                        <div>
                            Scanned: {{ item?.getDescription() }}
                        </div>
                        <div>
                            <button class="btn btn-sm btn-success me-2" @click="item.checkIn()"
                                v-if="item?.checkedOutAt()"><i class="bi-download"></i>
                                Check in</button>
                            <button class="btn btn-sm btn-primary me-2" @click="item.checkOut()" v-else><i
                                    class="bi-upload"></i>
                                Check out</button>
                            <button class="btn btn-sm btn-secondary" @click="edit"><i class="bi-pencil"></i>
                                Edit</button>
                        </div>
                    </div>

                    <button class="btn btn-secondary btn-sm me-2" @click="close"><i class="bi-x-lg"></i> Cancel</button>
                </div>
            </div>
        </div>
    </Transition>
</template>

<style>
.modal-mask {
    position: fixed;
    z-index: 9998;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    transition: opacity 0.3s ease;
}

.modal-enter-from {
    opacity: 0;
}

.modal-leave-to {
    opacity: 0;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
    -webkit-transform: scale(1.1);
    transform: scale(1.1);
}
</style>