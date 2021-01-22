<template>
  <div class="container">
    <div class="taskbar">
      <p class="click" @click="settingsOpen = !settingsOpen">Settings</p>
      <p class="click" @click="exit">CLOSE</p>
    </div>
    <Home v-if="!settingsOpen" class="home" :layout="layout" :update="update" />
    <Settings v-if="settingsOpen" />

  </div>
</template>

<script>
import Settings from "./view/Settings";
import Home from "./view/Home";
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
  data() {
    return {
      settingsOpen: false
    }
  },
  methods: {
    exit() {
      if(this.settingsOpen) {
        this.settingsOpen = !this.settingsOpen
      }else{
        exit()
      }
    }
  },
  components: {
    Home,
    Settings
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

.taskbar {
  display: flex;
  justify-content: right;

  >* {
  padding-left: 16px;
}
}

.container {
  display: grid;
  grid-template-rows: max-content 1fr;
  top: 0;
  width: 100%;
  height: 100%;
}

.justify-center {
  justify-self: center;
}

.align-center {
  align-self: center;
}

.click {
  cursor: pointer;
}

.title {
  display: inline-block;
}

html, body {
  background-color: black;
  color: white;
	height: 100vh;
	// overflow: hidden;
}
</style>