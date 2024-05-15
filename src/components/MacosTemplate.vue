<template>
  <span v-if="macos" style="display: flex; flex-direction: row; align-items: center">
    <b>暗色反转：</b>
    <label class="switch">
      <input type="checkbox" id="toggle-switch" @change="changeValue" />
      <span class="slider round"></span>
    </label>
  </span>
</template>

<script setup lang="ts">
import { cmds } from "@/plugins";
import { onMounted, ref } from "vue";

const macos = ref();

onMounted(async () => {
  const { os } = await cmds.app_info();
  macos.value = "macos" == os;
});

const changeValue = (e) => {
  cmds.set_template_icon(e.target.checked)
}
</script>

<style scoped>
.switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 34px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.4s;
  border-radius: 34px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 26px;
  width: 26px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: 0.4s;
  border-radius: 50%;
}

input:checked+.slider {
  background-color: var(--border-color);
}

input:focus+.slider {
  box-shadow: 0 0 1px var(--border-color);
}

input:checked+.slider:before {
  transform: translateX(26px);
}

/* Rounded sliders */
.slider.round {
  border-radius: 34px;
}

.slider.round:before {
  border-radius: 50%;
}
</style>
