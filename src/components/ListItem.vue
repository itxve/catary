<template>
  <div style="display: flex; flex-direction: column;">
    <div style="position: relative;">
      <img @click="imgClick"
        style=" width: 64px;height: 64px;  border-radius: 5px; border: 1px solid var(--border-color); cursor: pointer;;"
        :src="imageUrl" alt="加载失败" :title="props.desc" />
      <span class="img-select" v-if="$props.isCurrent" />
    </div>
    <span style="color: var(--border-color); cursor: pointer;" @click="delGif">删除</span>
  </div>
</template>


<script setup lang='ts'>
import { ref, onMounted } from 'vue';
import { useHomeFile, useObjectUrl } from '@/hooks';
import cmds from '@/plugins/cmds';

const props = defineProps(["id", "desc", "isCurrent"])

const emit = defineEmits(["del"]);

const imageUrl = ref();
const { readBinaryFile2Home } = useHomeFile();
const { createObjectURL } = useObjectUrl();

onMounted(async () => {
  if (props.id) {
    const file = await readBinaryFile2Home(props.id + ".gif")
    imageUrl.value = createObjectURL(new Blob([file]))
  }
})


const imgClick = async () => {
  cmds.set_current_gif({ id: props.id, desc: props.desc });
  emit("del");
}

const delGif = () => {
  cmds.del_current_gif({ id: props.id, desc: props.desc });
  emit("del");
}


</script>

<style scoped>
.inline-block {
  display: inline-block;
}

dialog {
  border-radius: 5px;
  border-color: var(--border-color);
}

.img-select {
  position: absolute;
  width: 10px;
  height: 10px;
  right: 0px;
  background-color: rgba(234, 77, 10, 0.901);
  border-top-right-radius: 5px;
}
</style>