<template>
  <div class="app">
    <div v-if="!comparisonResult">
      <FileUpload @submit="handleFileSubmit" />
      <div v-if="headers.length > 0" class="header-selection">
        <label for="header-select">Select comparison header:</label>
        <select id="header-select" v-model="selectedHeader">
          <option v-for="header in headers" :key="header" :value="header">
            {{ header }}
          </option>
        </select>
        <button @click="compareFiles" :disabled="!selectedHeader || files.length !== 2">
          Compare Files
        </button>
      </div>
    </div>
    <div v-else-if="error" class="error-message">
      {{ error }}
    </div>
    <Card v-else>
      <template #title>
        Comparison Results
      </template>
      <template #content>
        <ComparisonTable :comparisonData="comparisonResult" />
      </template>
      <template #footer>
        <Button @click="resetComparison" label="Compare New Files" />
      </template>
    </Card>
  </div>
</template>

<script>
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/core";
import FileUpload from './components/FileUpload.vue'
import ComparisonTable from './components/ComparisonTable.vue'

export default {
  name: 'App',
  components: {
    FileUpload,
    ComparisonTable
  },
  setup() {
    const files = ref([])
    const headers = ref([])
    const selectedHeader = ref('')
    const comparisonResult = ref(null)
    const error = ref(null)

    const handleFileSubmit = async (selectedFiles) => {
      files.value = selectedFiles
      if (files.value.length > 0) {
        const content = await readFileContent(files.value[0])
        headers.value = getHeadersFromCSV(content)
      }
    }

    const getHeadersFromCSV = (content) => {
      const lines = content.split('\n')
      if (lines.length > 0) {
        return lines[0].split(',').map(header => header.trim())
      }
      return []
    }

    const compareFiles = async () => {
      if (files.value.length !== 2 || !selectedHeader.value) return

      error.value = null
      comparisonResult.value = null

      const content1 = await readFileContent(files.value[0])
      const content2 = await readFileContent(files.value[1])

      try {
        const result = await invoke('compare_csv_contents', { 
          file1Content: content1,
          file2Content: content2,
          comparisonHeader: selectedHeader.value
        })
        
        if (result && typeof result === 'object' && 'changed_rows' in result) {
          comparisonResult.value = result
        } else {
          throw new Error('Unexpected result format from comparison')
        }
      } catch (err) {
        error.value = `Error comparing files: ${err.message || 'Unknown error'}`
      }
    }

    const readFileContent = (file) => {
      return new Promise((resolve, reject) => {
        const reader = new FileReader()
        reader.onload = (event) => resolve(event.target.result)
        reader.onerror = (error) => reject(error)
        reader.readAsText(file)
      })
    }

    const resetComparison = () => {
      files.value = []
      headers.value = []
      selectedHeader.value = ''
      comparisonResult.value = null
      error.value = null
    }

    return {
      files,
      headers,
      selectedHeader,
      comparisonResult,
      error,
      handleFileSubmit,
      compareFiles,
      resetComparison
    }
  }
}
</script>

<style scoped>
.app {
  font-family: Arial, sans-serif;
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.header-selection {
  margin-top: 20px;
}

button {
  margin-top: 10px;
}

.error-message {
  color: red;
  font-weight: bold;
  margin: 20px 0;
}
</style>