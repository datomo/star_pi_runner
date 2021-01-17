<template>
  <div class="task-list">
    <h1>{{msg + 't'}}</h1>
    <h1>tess</h1>
    <div
      :class="isDone(task)"
      v-for="(task, index) in tasks"
      :key="`task.name-${index}`"
      @click="_markTask(index, task.done)"
    >{{task.name}}</div>
  </div>
</template>

<script>
import { markTask, addCallback } from "../rpc";

export default {
  props: {
    tasks: {
      type: Array,
      required: true,
    },
    msg: {
      type: String,
      required: true
    }
  },
  mounted () {
    addCallback(this.update);
  },
  methods: {
    isDone: function(task) {
      let checked = task.done ? "checked" : "unchecked";
      return "task-item " + checked;
    },
    _markTask: function(i, done) {
      markTask(i, !done);
    },
  }
};
</script>

<style scoped>
.task-list {
  overflow-y: auto;
  position: fixed;
  top: 2.5em;
  bottom: 1.2em;
  left: 0;
  right: 0;
}
.task-item {
  height: 1.5em;
  color: rgba(255, 255, 255, 0.87);
  padding: 0.5em;
  cursor: pointer;
}
.task-item.checked {
  text-decoration: line-through;
  color: rgba(255, 255, 255, 0.38);
}
</style>