<template>
  <div class="scale">
    <div class="textual">
      <h3>{{value}} {{unit}}</h3>
    </div>
    <div class="visual">
      <div class="bar">
        <div class="filler" :style="{width: width}"></div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "Scale",
  props: {
    data: {
      type: Object
    },
    unit: {
      type: String,
      default: "g"
    }
  },
  computed: {
    max() {
      if ("max" in this.data) {
        return this.data["max"];
      } else {
        return 0;
      }
    },
    value() {
      if ("value" in this.data) {
        return this.data["value"];
      } else {
        return 0;
      }
    },
    width() {
      return this.value/this.max*100;
    }
  }
}
</script>

<style lang="scss" scoped>
  .bar{
    height: 40px;
    width: 25vw;
    position: relative;
    background: grey;

    .filler {
      position: absolute;
      left: 0;
      top: 0;
      height: 100%;
      background: blue;
    }
  }
</style>