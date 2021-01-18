import Vue from 'vue';
import App from './App.vue';
import { init } from './rpc';

let vm = new Vue({
  el: "#app",
  data: function () {
    return {
      update: {},
      layout: {}
    }
  },
  render: function (h) {
    return h(App, { attrs: { update: this.update, layout: this.layout } })
  }
});

window.onload = function () { init(); };

function fromRust(update) {
  // we only update where we got new information
  Object.keys(update).forEach(k => {
      vm.update[k] = update[k];
  })
}

function sendLayout(layout) {
  vm.layout = layout;
}

export { fromRust, sendLayout };