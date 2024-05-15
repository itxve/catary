<template>
  <div>
    <div style=" display: flex;
        flex-direction: row;
        gap: 20px;
        flex-wrap: wrap;align-content: center;">
      <Speed />
      <MacosTemplate />
      <GifItemLoad @success="onSuccess" :edit="false" :show="showDialog" @close="onClose" />
      <button style="transform: scale(0.8)" @click="() => (showDialog = true)">
        添加
      </button>
    </div>


    <div style="
        height: 425px;
        display: flex;
        flex-direction: row;
        gap: 8px;
        flex-wrap: wrap;
      ">
      <ListItem :key="item.id" v-for="item in list" :id="item.id" :desc:="item.desc" :isCurrent="item.id == currentRef"
        @del="onSuccess" />
    </div>
  </div>
</template>

<script setup lang="ts">
import GifItemLoad from "@/components/GifItemLoad.vue";
import ListItem from "@/components/ListItem.vue";
import MacosTemplate from "@/components/MacosTemplate.vue";
import Speed from "@/components/Speed.vue";

import { onMounted, ref } from "vue";
import { cmds } from "@/plugins";

const list = ref();
const currentRef = ref();
const showDialog = ref(false);

const loadUserGifs = async () => {
  list.value = await cmds.user_gifs();
  currentRef.value = (await cmds.app_info()).current_gif;
};

const onSuccess = () => {
  loadUserGifs();
  showDialog.value = false;
};

const onClose = () => {
  showDialog.value = false;
};

onMounted(async () => {
  loadUserGifs();
});
</script>

<style scoped></style>
