<template>
  <div class="main">
    <Button as="a" href="https://github.com/xdannyrobertsx/csv-compare-js" target="_blank" rel="noopener" icon="pi pi-github" rounded severity="secondary" style="text-decoration: none" />
    <div v-if="comparisonResult">
      <!-- Display the comparison result here -->
    </div>
    <FileUpload v-else :isCalculating @submit="compareFiles" />
  </div>
</template>

<script setup>
import { ref, computed } from "vue";
import { invoke } from '@tauri-apps/api/core';
import FileUpload from "./components/FileUpload.vue";

const isCalculating = computed(() => {
  return false;
});

const files = ref([]);
const comparisonResult = ref(null);

const compareFiles = async (e) => {
  files.value = e;
  const fileContents = await Promise.all(
    Array.from(files.value).map(file => readFileContent(file))
  );

  try {
    comparisonResult.value = await invoke('compare_csv_contents', {
      file1Content: fileContents[0],
      file2Content: fileContents[1]
    });
    // Handle the comparison result here
    console.log("✨", comparisonResult.value);
  } catch (error) {
    console.error('Error comparing files:', error);
  }
};

  const readFileContent = (file) => {
    return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = event => resolve(event.target.result);
    reader.onerror = error => reject(error);
    reader.readAsText(file);
  });
};
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
