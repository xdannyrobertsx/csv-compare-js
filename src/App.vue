<template>
  <div class="main">
    <Button
      as="a"
      href="https://vuejs.org/"
      target="_blank"
      rel="noopener"
      icon="pi pi-github"
      rounded
      severity="secondary"
      style="text-decoration: none"
    />
    <FileUpload :isCalculating @submit="handleSubmit"/>
  </div>
</template>

<script setup>
import { computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import FileUpload from "./components/FileUpload.vue";

const isCalculating = computed(() => {
  return false;
});

const handleSubmit = (files) => {
  console.log(files)
}


// const file = await open({
//   multiple: false,
//   directory: false,
// });
// console.log(file);
</script>

<style scoped>
.main {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 90vh;
}
</style>
