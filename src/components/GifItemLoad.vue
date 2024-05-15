<template>
  <div v-if="props.show" style=" position:fixed;  top: 10vh; left: 5vw ;z-index: 100;">
    <dialog :open="props.show" class="content">
      <span class="close" @click="() => emit('close')" />
      <div style="display: flex; flex-direction: row; margin-bottom: 32px">
        <DragFile :src="imageUrl" @file="fileChange" style="width: 100px" />
        <div>
          <div>
            <label style="width: 100%">🎫 <b style="font-size: 14px">{{ uidRef }}</b></label>
          </div>
          <div>
            <label>描述:</label><input v-model="descRef" style="border: indigo solid 1px" placeholder="描述" />
          </div>
        </div>
      </div>
      <button class="save" style="scale: .8;" @click="saveImage">保存</button>
    </dialog>
  </div>
</template>

<script setup lang="ts">
import DragFile from "@/components/DragFile.vue";
import { onMounted, ref } from "vue";
import { loadBinaryFile } from "@/util";
import { cmds } from "@/plugins";
import { genId } from "@/util";
import { useHomeFile } from "@/composable";

const props = defineProps(["edit", "uid", "show"]);
const emit = defineEmits(["success", "close"]);

const imageUrl = ref();
const fileRef = ref<ArrayBuffer>();
const uidRef = ref();
const descRef = ref();
const { writeBinaryFile2Home } = useHomeFile();

onMounted(() => {
  if (!props.edit) {
    uidRef.value = genId();
  } else {
    uidRef.value = props.uid;
  }
});


const fileChange = async (files: Array<File>) => {
  cmds.set_template_icon(false);
  const file = files && files[0];
  if (file && file.type == "image/gif") {
    const file_byte = await loadBinaryFile(file);
    cmds.change_tary(Array.from(new Uint8Array(file_byte)));
    imageUrl.value = URL.createObjectURL(new Blob([file]));
    fileRef.value = new Uint8Array(file_byte);
  }
};

const saveImage = async () => {
  if (!fileRef.value) {
    return;
  }
  await writeBinaryFile2Home(`${uidRef.value}.gif`, fileRef.value);
  await cmds.add_image({ id: `${uidRef.value}`, desc: descRef.value });
  uidRef.value = genId();
  descRef.value = "";
  imageUrl.value = "";
  emit("success", true);
};
</script>

<style scoped>
.content {
  position: relative;
}

.content>.close {
  cursor: pointer;
  position: absolute;
  right: 0px;
  top: 0px;
  transform: scale(0.8);
}

.content>.close::after {
  content: "❌";
  filter: grayscale();
}

.inline-block {
  display: inline-block;
}

dialog {
  border-radius: 5px;
  border-color: var(--border-color);
}

.save {
  position: absolute;
  right: 0px;
  bottom: 0px;
  background-color: var(--border-color);
  color: white;
}
</style>
