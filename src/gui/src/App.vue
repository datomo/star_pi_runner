<template>
  <div class="container">
    {{"layout: " + JSON.stringify(layout)}}
    {{"scales: " + JSON.stringify(scales)}}
    <div v-for="scale in scales" :key="scale">
      <scale :update="update[scale]" />
    </div>

  </div>
</template>

<script>
import Scale from "./components/Scale";

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


<style>
* {
	margin: 0;
	padding: 0;
	box-sizing: border-box;
	font-size: 28px;
	font-family: sans-serif;
}

html, body {
	height: 100%;
	overflow: hidden;
}
</style>