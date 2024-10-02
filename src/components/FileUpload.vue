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
        disabled: shouldShowUpload,
        severity: 'contrast',
      }"
      :uploadButtonProps="{
        label: 'Compare',
        iconPos: 'left',
        disabled: !shouldShowUpload,
        loading: isCalculating,
        severity: 'primary',
      }"
      :disabled="isCalculating"
      :multiple="true"
      :maxFileSize="1000000"
      :fileLimit="2"
      :customUpload="true"
      :showUploadButton="shouldShowUpload"
      :showCancelButton="shouldShowCancel"
      :previewWidth="0"
    >
      <template #empty>
        <span>Drag and drop files to here to upload.</span>
      </template>
      <template #chooseicon>
        <i
          :class="['pi', shouldShowUpload ? 'pi-folder' : 'pi-folder-open']"
        ></i>
      </template>
      <template #uploadicon>
        <i
          :class="['pi', isCalculating ? 'pi-spinner-dotted' : 'pi-sparkles']"
        ></i>
      </template>
    </FileUpload>
  </div>
</template>

<script setup>
import { ref, computed, defineProps, defineEmits } from "vue";


const emit = defineEmits(['submit'])

defineProps({
  isCalculating: {
    type: Boolean,
    default: false,
  }
})

const filesInQueue = ref(0);

const calculateFilesInQueue = (uploadEvent) => {
  const { files } = uploadEvent;
  filesInQueue.value = files.length;
};

const handleFiles = (uploadEvent) => {
  filesInQueue.value = 0;

  const { files } = uploadEvent;
  emit('submit', files)
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
