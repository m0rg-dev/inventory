<script setup>
import { Html5QrcodeScanner } from "html5-qrcode";
</script>

<script>
export default {
  mounted() {
    const config = {
      fps: 10,
    };
    const html5QrcodeScanner = new Html5QrcodeScanner(
      "qr-code-full-region",
      config
    );
    html5QrcodeScanner.render(this.onScanSuccess);
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