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
  const temp = JSON.parse(JSON.stringify(vm.update));
  for( const k of Object.keys(update)){
    temp[k] = update[k];
  }
  vm.update = temp
}

function sendLayout(layout) {
  vm.layout = layout;
}

export { fromRust, sendLayout };