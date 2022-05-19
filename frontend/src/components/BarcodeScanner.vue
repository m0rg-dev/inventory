<script setup>
import { Html5QrcodeScanner } from "html5-qrcode";
</script>

<script>
export default {
  data: () => ({
    scanner: null
  }),
  mounted() {
    const config = {
      fps: 10,
    };
    this.scanner = new Html5QrcodeScanner(
      "qr-code-full-region",
      config
    );
    this.scanner.render(this.onScanSuccess);
  },
  unmounted() {
    this.scanner.clear();
  },
  methods: {
    onScanSuccess(decodedText, decodedResult) {
      this.$emit("result", decodedText, decodedResult);
    },
  },
};
</script>

<template>
  <div id="qr-code-full-region"></div>
</template>