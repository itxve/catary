<template>
  <div>
    <div style="display: flex;flex-direction: row; gap: 8px; flex-wrap: wrap;">
      <ListItem :key="item.id" v-for="item in list" :id="item.id" :desc:="item.desc" :isCurrent="item.id == currentRef"
        @del="onSuccess" />
    </div>
    <GifItemLoad @success="onSuccess" :edit="false" />
  </div>
</template>

<script setup lang="ts">
import GifItemLoad from "@/components/GifItemLoad.vue";
import ListItem from "@/components/ListItem.vue";
import { onMounted, ref } from 'vue';
import { cmds } from '@/plugins';

const list = ref()
const currentRef = ref()

const loadUserGifs = async () => {
  list.value = await cmds.user_gifs()

  currentRef.value = (await cmds.app_info()).current_gif
  console.log(" currentRef.value", currentRef.value);

}

const onSuccess = () => {
  loadUserGifs()
}

onMounted(async () => {
  loadUserGifs()
})
</script>



<style scoped></style>
