<template>
  <div class="scale">
    <h1 class="title">Scale</h1>
    <div class="textual">
      <h3>{{ value + unit }}</h3>
    </div>
    <div class="visual">
      <div class="bar">
        <div class="filler" :style="{width: width + 'px'}"></div>
      </div>
      <p class="min">0</p>
      <p class="max">{{ max }}</p>
    </div>
  </div>
</template>

<script>
export default {
  name: "Scale",
  props: {
    update: {
      type: Object
    },
    unit: {
      default: "g"
    }
  },
  computed: {
    max() {
      if (this.update !== undefined && this.update.hasOwnProperty("max")) {
        return this.update["max"];
      } else {
        return 0;
      }
    },
    value() {
      if (this.update !== undefined && this.update.hasOwnProperty("value")) {
        return this.update["value"];
      } else {
        return 0;
      }
    },
    width() {
      return this.value / this.max * 100;
    }
  }
}
</script>

<style lang="scss" scoped>
.scale {
  display: inline-block;
}

.visual {
  display: grid;
  grid-template-areas: "bar bar" "min max";

  .min {
    grid-area: min;
  }

  .max {
    grid-area: max;
    justify-self: right;
  }
}

.bar {
  height: 40px;
  width: 50vw;
  position: relative;
  grid-area: bar;
  background: grey;
}

.filler {
  position: absolute;
  left: 0;
  top: 0;
  height: 100%;
  background: blue;
}
</style>