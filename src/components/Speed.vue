<template>

  <div style="position: relative; display: flex; align-items: center;">
    <label class="roket" :style="{
      position: 'absolute',
      cursor: 'pointer', pointerEvents: 'none', left: `${leftV}px`, scale: '.8', transition: 'all 0.2s'
    }"></label>
    <div class="progress-container" ref="progressDom" style="cursor: pointer; ">
      <div class="progress-bar" :style="{ width: `${leftV}px` }" />
    </div>
  </div>
</template>



<script setup lang='ts'>
import { onMounted, onUnmounted, ref } from 'vue';
import { cmds } from '@/plugins';
const progressDom = ref<HTMLProgressElement>()
const progressV = ref(0)
const leftV = ref(-5)
const enter = ref(false)

const click = (e) => {
  progressDom.value!.value = e.offsetX
  leftV.value = e.offsetX
  cmds.set_speed_millis((100 - e.offsetX) > 0 ? 100 - e.offsetX : 10)
}
const mousedown = () => {
  enter.value = true
}
const mousemove = (e: MouseEvent) => {

  if (!enter.value) {
    return
  }
  leftV.value = e.offsetX
}
const mouseup = (e) => {
  enter.value = false
  progressV.value = e.offsetX
  setSpeed()
}

const bodyMoveUp = (e) => {
  progressV.value = e.target.offsetX <= 100 ? 100 : e.target.offsetX
  enter.value = false
}

const setSpeed = () => {
  let millis = (100 - progressV.value) > 0 ? 100 - progressV.value : 10
  cmds.set_speed_millis(millis)
}

onMounted(() => {
  progressDom.value!.addEventListener("click", click)
  progressDom.value!.addEventListener("mousedown", mousedown)
  progressDom.value!.addEventListener("mousemove", mousemove)
  progressDom.value!.addEventListener("mouseup", mouseup)
  document.querySelector("body")!.addEventListener("mouseup", bodyMoveUp)
})
onUnmounted(() => {
  progressDom.value!.removeEventListener("click", click)
  progressDom.value!.removeEventListener("mousedown", mousedown)
  progressDom.value!.removeEventListener("mousemove", mousemove)
  progressDom.value!.removeEventListener("mouseup", mouseup)
  document.querySelector("body")!.removeEventListener("mouseup", bodyMoveUp)

})

</script>

<style scoped>
.roket::after {
  content: '🚀';
  left: -10px;
}

/* 进度条容器 */
.progress-container {
  width: 100px;
  background-color: #c3b7c5;
  /* 进度条背景颜色 */
}

/* 进度条 */
.progress-bar {
  height: 8px;
  background-color: var(--border-color);
  text-align: center;
  line-height: 8px;
  width: 0%;
  transition: width 0.2s;
}
</style>