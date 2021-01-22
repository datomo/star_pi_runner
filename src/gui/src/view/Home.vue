<template>
  <div class="home">
    <h1>Home</h1>
    <div class="justify-center align-center" v-for="scale in scales" :key="scale">
      <scale :update="update[scale]"/>
    </div>
  </div>
</template>

<script>
import Scale from "../components/Scale";

export default {
  name: "Home",
  props: {
    update: {
      type: Object,
      required: true
    },
    layout: {
      type: Object,
      required: true
    }
  },
  components: {
    Scale
  },
  computed: {
    scales() {
      if ("scales" in this.layout) {
        return this.layout["scales"];
      } else {
        return [];
      }
    }
  },
}
</script>

<style lang="scss" scoped>
.home {
  display: grid;
  height: 100%;
  width: 100%;
}
</style>