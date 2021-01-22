<template>
  <div class="container">
    <p class="click" @click="exit">CLOSE</p>
    <div class="justify-center" v-for="scale in scales" :key="scale">
      <scale :update="update[scale]" />
    </div>

  </div>
</template>

<script>
import Scale from "./components/Scale";
import {exit} from "./rpc";

export default {
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
  methods: {
    exit() {
      exit()
    }
  },
  components: {
    Scale,
  },
  computed: {
    scales() {
      if ( "scales" in this.layout ) {
        return this.layout["scales"];
      }else {
        return [];
      }
    }
  },
};
</script>


<style lang="scss">
* {
	margin: 0;
	padding: 0;
	box-sizing: border-box;
	font-size: 28px;
	font-family: sans-serif;
}

.container {
  display: grid;
  width: 100%;
}

.justify-center {
  justify-self: center;
}

.click {
  cursor: pointer;
}

.title {
  display: inline-block;
}

html, body {
	height: 100%;
	overflow: hidden;
}
</style>