<template>
  <div class="main">
    <div v-if="!comparisonResult">
      <FileUpload @submit="handleFileSubmit" />
      <div v-if="headers.length > 0" class="header-selection">
        <Select
          v-model="selectedHeader"
          :options="headers"
          placeholder="Select comparison header"
          class="dropdown"
        />
        <Select
          v-model="oldestFile"
          :options="files.map((f) => f.name)"
          placeholder="Select oldest file:"
          class="dropdown"
        />
        <Button
          @click="compareFiles"
          :disabled="!selectedHeader || files.length !== 2"
          label="Compare Files"
        />
      </div>
    </div>
    <div v-else-if="error" class="error-message">
      {{ error }}
    </div>
    <Card v-else>
      <template #title> Comparison Results </template>
      <template #content>
        <ComparisonTable :comparisonData="comparisonResult" />
      </template>
      <template #footer>
        <div class="footer-container">
        <Button @click="resetComparison" label="Compare New Files" />
        <Button @click="downloadCsv" label="Download CSV" />
      </div>
      </template>
    </Card>
  </div>
</template>

<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { create, BaseDirectory } from "@tauri-apps/plugin-fs";
import { ask, save } from "@tauri-apps/plugin-dialog";
import FileUpload from "./components/FileUpload.vue";
import ComparisonTable from "./components/ComparisonTable.vue";

const files = ref([]);
const oldestFile = ref("");
const headers = ref([]);
const selectedHeader = ref("");
const comparisonResult = ref(null);
const error = ref(null);

const oldestFileContent = computed(() => {
  return files.value.find((file) => file.name === oldestFile.value);
});

const newestFileContent = computed(() => {
  return files.value.find((file) => file.name !== oldestFile.value);
});

const handleFileSubmit = async (selectedFiles) => {
  files.value = selectedFiles;
  if (files.value.length > 0) {
    const content = await readFileContent(files.value[0]);
    headers.value = getHeadersFromCSV(content);
  }
};

const getHeadersFromCSV = (content) => {
  const lines = content.split("\n");
  if (lines.length > 0) {
    return lines[0].split(",").map((header) => header.trim());
  }
  return [];
};

const compareFiles = async () => {
  if (files.value.length !== 2 || !selectedHeader.value) return;

  const userConfirm = await ask("Are you sure you'd like to compare these two files?", {
    title: "CSV Compare",
    kind: "warning",
  });

  if (!userConfirm) return;

  error.value = null;
  comparisonResult.value = null;

  const content1 = await readFileContent(oldestFileContent.value);
  const content2 = await readFileContent(newestFileContent.value);

  try {
    const result = await invoke("compare_csv_contents", {
      file1Content: content1,
      file2Content: content2,
      comparisonHeader: selectedHeader.value,
    });

    if (result && typeof result === "object") {
      comparisonResult.value = result;
    } else {
      throw new Error("Unexpected result format from comparison");
    }
  } catch (err) {
    error.value = `Error comparing files: ${err.message || "Unknown error"}`;
  }
};

const readFileContent = (file) => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (event) => resolve(event.target.result);
    reader.onerror = (error) => reject(error);
    reader.readAsText(file);
  });
};

const resetComparison = () => {
  files.value = [];
  oldestFile.value = "";
  headers.value = [];
  selectedHeader.value = "";
  comparisonResult.value = null;
  error.value = null;
};

const downloadCsv = async () => {
  if (!comparisonResult.value) {
    return;
  }

  // Get the original file names provided by the user
  const oldFileName = files.value[0].name;
  const newFileName = files.value[1].name;

  // Collect all unique headers from the records
  const allHeaders = new Set();

  // Collect headers from added, removed, and changed rows
  const collectHeaders = (row) => {
    Object.keys(row.record || row.changes).forEach((key) => {
      allHeaders.add(key);
    });
  };

  comparisonResult.value.added_rows.forEach(collectHeaders);
  comparisonResult.value.removed_rows.forEach(collectHeaders);
  comparisonResult.value.changed_rows.forEach(collectHeaders);

  // Convert the Set of headers into an Array and sort them
  const headers = Array.from(allHeaders).sort();

  // Ensure the comparisonHeader (e.g., "ID") is the first column
  const comparisonHeader = selectedHeader.value;
  const headerIndex = headers.indexOf(comparisonHeader);
  if (headerIndex !== -1) headers.splice(headerIndex, 1);
  headers.unshift(comparisonHeader);

  // Add Source and Message as the last columns
  headers.push("Source", "Message");

  // Prepare CSV content with dynamic headers
  let csvContent = [headers.join(",")];

  // Helper function to create a CSV row from a record
  const createCsvRow = (id, record, source, message) => {
    return headers.map((header) => {
      let value = "";

      if (header === comparisonHeader) {
        value = id || "";
      } else if (header === "Source") {
        value = source || "";
      } else if (header === "Message") {
        value = message || "";
      } else {
        value = record[header] || "";
      }

      // Escape commas and quotes for CSV
      if (value.includes(",") || value.includes('"')) {
        value = `"${value.replace(/"/g, '""')}"`;
      }

      return value;
    }).join(",");
  };

  // Iterate over added rows
  for (const row of comparisonResult.value.added_rows) {
    const csvRow = createCsvRow(
      row.id,
      row.record,
      newFileName,
      "Record only in new file"
    );
    csvContent.push(csvRow);
  }

  // Iterate over removed rows
  for (const row of comparisonResult.value.removed_rows) {
    const csvRow = createCsvRow(
      row.id,
      row.record,
      oldFileName,
      "Record only in old file"
    );
    csvContent.push(csvRow);
  }

  // Iterate over changed rows
  for (const row of comparisonResult.value.changed_rows) {
    const fieldsChanged = Object.keys(row.changes)
      .filter((key) => key !== comparisonHeader)
      .join(", ");

    // Create two rows: one for the old value and one for the new value
    const oldCsvRow = createCsvRow(
      row.id,
      Object.fromEntries(
        Object.entries(row.changes).map(([key, value]) => [key, Array.isArray(value) ? value[0] : value])
      ),
      oldFileName,
      `Changed fields: ${fieldsChanged}`
    );

    const newCsvRow = createCsvRow(
      row.id,
      Object.fromEntries(
        Object.entries(row.changes).map(([key, value]) => [key, Array.isArray(value) ? value[1] : value])
      ),
      newFileName,
      `Changed fields: ${fieldsChanged}`
    );

    csvContent.push(oldCsvRow);
    csvContent.push(newCsvRow);
  }

  // Convert CSV content to a single string
  csvContent = csvContent.join("\n");

  try {
    const filePath = await save({
      filters: [{ name: "", extensions: ["csv"] }],
    });

    const file = await create(filePath);
    await file.write(new TextEncoder().encode(csvContent));
    await file.close();
  } catch (err) {
    console.error("Error saving CSV file:", err);
  }
};
</script>

<style scoped>
.main {
  font-family: Arial, sans-serif;
  max-width: 1000px;
  margin: 0 auto;
  padding: 50px;
}

.header-selection {
  margin-top: 20px;
  text-align: center;
  width: 100%;
}

.header-selection > * {
  max-width: 400px;
  width: 100%;
  margin: 10px;
}

button {
  margin-top: 10px;
}

.footer-container {
  display: flex;
  justify-content: space-between;
}

.error-message {
  color: red;
  font-weight: bold;
  margin: 20px 0;
}
</style>
