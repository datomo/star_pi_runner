import Vue from 'vue';
import App from './App.vue';
import { init } from './rpc';

let vm = new Vue({
  el: "#app",
  data: function () {
    return {
      tasks: [],
      msg: "dus"
    }
  },
  render: function (h) {
    return h(App, { attrs: { tasks: this.tasks, msg: this.msg } })
  }
});

window.onload = function () { init(); };

function fromRust(tasks) {
  vm.tasks = tasks;
}

function testing(msg) {
  vm.msg = msg;
}

export { fromRust, testing };