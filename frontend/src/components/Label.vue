<script setup>
import bwipjs from "bwip-js";
</script>

<script>
// https://stackoverflow.com/a/16599668
function getLines(ctx, text, maxWidth) {
  if (text == null) {
    text = "";
  }
  let words = text.split(" ");
  let lines = [];
  let currentLine = words[0];

  for (let i = 1; i < words.length; i++) {
    let word = words[i];
    let width = ctx.measureText(currentLine + " " + word).width;
    if (width < maxWidth) {
      currentLine += " " + word;
    } else {
      lines.push(currentLine);
      currentLine = word;
    }
  }
  lines.push(currentLine);
  return lines;
}

export default {
  props: {
    id: {
      type: String,
    },
    desc: {
      type: String,
    },
  },

  data: () => ({
    canvas: null
  }),

  watch: {
    id() {
      this.update();
    },
    desc() {
      this.update();
    },
  },

  mounted() {
    this.update();
  },

  methods: {
    update() {
      const barcode_canvas = document.createElement("canvas");
      bwipjs.toCanvas(barcode_canvas, {
        bcid: "qrcode",
        text: this.id.toUpperCase(),
        scale: 5,
        rotate: "L",
      });

      const canvas = document.createElement("canvas");
      canvas.height = 304 * 2;
      canvas.width = 304;

      const ctx = canvas.getContext("2d");
      ctx.fillStyle = "#eee";
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      ctx.drawImage(
        barcode_canvas,
        (canvas.width - barcode_canvas.width) / 2,
        canvas.height - barcode_canvas.height
      );
      const size = 32;

      ctx.font = `${size}px monospace`;
      ctx.fillStyle = "black";
      ctx.textAlign = "left";

      ctx.save();
      ctx.translate(50, canvas.height - barcode_canvas.height - 30);
      ctx.rotate(-3.14159265 / 2);

      const lines = getLines(
        ctx,
        this.desc,
        canvas.height - barcode_canvas.height - 60
      );

      for (let i = 0; i < lines.length; i++) {
        ctx.fillText(lines[i], 0, size * i);
      }

      ctx.restore();

      this.canvas = canvas;

      document.getElementById("label").src = canvas.toDataURL("image/png");
    },

    async print() {
      this.canvas.toBlob((b) => {
        fetch(`/api/label`, {
          method: "POST",
          headers: {
            "Content-Type": "image/png"
          },
          body: b
        });
      }, "image/png");
    }
  },
};
</script>

<template>
  <div>
    <button class="btn btn-primary btn-sm mb-2" @click="print"><i class="bi-printer"></i> Print label</button>
    <div><img id="label" style="width: 100px; height: auto" /></div>
  </div>
</template>