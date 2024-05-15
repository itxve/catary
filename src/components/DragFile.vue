<template>
  <div>
    <input type="file" @change="fileChange" v-show="false" ref="selectInput" />
    <div @click="() => openFileSelect()"
      :class="['drop-zone', { draging: dragging && props.src, 'draging-del': props.src }]" @dragenter="dragenterEvent"
      @dragover="dragoverEvent" @dragleave="dragleaveEvent" @drop="dropEvent">
      <div v-if="!props.src">
        <span v-if="dragging">松开以放置</span>
        <span v-else style="text-align: center;">选择/拖入gif图片</span>
      </div>
      <div v-else class="img-show">
        <img style="width: 80px;height: 80px; border-radius: 5px;" :src="props.src" alt="加载失败">
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from "vue";

const props = defineProps(['src'])
const dragging = ref(false);
const selectInput = ref();
const emit = defineEmits(["file"]);

const openFileSelect = () => {
  selectInput.value?.click();
};
const fileChange = (event) => {
  emit("file", Array.from(event.target.files));
};

function dragenterEvent(event) {
  event.stopPropagation();
  event.preventDefault();
  dragging.value = true;
}
function dragoverEvent(event) {
  event.stopPropagation();
  event.preventDefault();
}
function dragleaveEvent(event) {
  event.stopPropagation();
  event.preventDefault();
  dragging.value = false;
}
function dropEvent(event) {
  event.stopPropagation();
  event.preventDefault();
  dragging.value = false;
  emit("file", Array.from(event.dataTransfer.files));
}
</script>

<style scoped>
.drop-zone {
  box-sizing: content-box;
  display: inline-block;
  align-content: center;
  cursor: pointer;
  width: 80px;
  height: 80px;
  text-align: center;
  color: #716d6d;
  border: 2px dashed grey;
}

.draging {
  background-color: #98bcd3;
}

.draging-del {
  border: none;
  border: 1px solid var(--border-color);
  border-radius: 5px;
}
</style>
