<template>
  <div class="card">
    <FileUpload
      @uploader="handleFiles"
      @select="calculateFilesInQueue"
      @remove="calculateFilesInQueue"
      @clear="filesInQueue = 0"
      accept=".csv"
      :chooseButtonProps="{
        label: 'Select Files',
        iconPos: 'right',
        severity: 'contrast',
        style: `display: ${shouldShowUpload ? 'none' : 'block'}`,
      }"
      :uploadButtonProps="{
        label: 'Choose Header',
        iconPos: 'left',
        loading: isCalculating,
        severity: 'primary',
        style: `display: ${shouldShowUpload ? 'block' : 'none'}`,
      }"
      :pt="passThroughOptions"
      :disabled="isCalculating"
      :multiple="true"
      :maxFileSize="1000000"
      :fileLimit="2"
      :customUpload="true"
      :showUploadButton="shouldShowUpload"
      :showCancelButton="shouldShowCancel"
    >
      <template #empty>
        <span>Drag and drop files to here to upload.</span>
      </template>
      <template #chooseicon>
        <i
          :class="['pi', shouldShowUpload ? 'pi-folder' : 'pi-folder-open']"
          style="padding-right: 8px"
        ></i>
      </template>
      <template #uploadicon>
        <i
          :class="['pi', isCalculating ? 'pi-spinner-dotted' : 'pi-search']"
          style="padding-right: 8px"
        ></i>
      </template>
    </FileUpload>
  </div>
</template>

<script setup>
import { ref, computed } from "vue";

const emit = defineEmits(["submit"]);

defineProps({
  isCalculating: {
    type: Boolean,
    default: false,
  },
});

const filesInQueue = ref(0);

const passThroughOptions = {
  header: {
    style: {
      "justify-content": "center",
    },
  },
  fileThumbnail: {
    style: {
      display: "none",
    },
  },
  root: {
    class: "danny",
  },
};

const calculateFilesInQueue = (uploadEvent) => {
  const { files } = uploadEvent;
  filesInQueue.value = files.length;
};

const handleFiles = (uploadEvent) => {
  filesInQueue.value = 0;

  const { files } = uploadEvent;
  emit("submit", files);
};

const shouldShowUpload = computed(() => {
  return filesInQueue.value > 1;
});
const shouldShowCancel = computed(() => {
  return filesInQueue.value > 0;
});
</script>
<style scoped>
.card {
  border-radius: 0.5rem;
  padding: 1.5rem;
}
</style>
