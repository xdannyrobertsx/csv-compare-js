<template>
  <div class="comparison-table">
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
    <table v-else-if="hasData">
      <thead>
        <tr>
          <th v-for="header in tableHeaders" :key="header">{{ header }}</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, index) in processedData" :key="index">
          <td v-for="header in tableHeaders" :key="header">
            <template v-if="row[header]">
              <template v-if="Array.isArray(row[header])">
                <span class="old-value">{{ row[header][0] }}</span>
                <span class="arrow">→</span>
                <span class="new-value">{{ row[header][1] }}</span>
              </template>
              <template v-else>
                {{ row[header] }}
              </template>
            </template>
          </td>
        </tr>
      </tbody>
    </table>
    <div v-else class="no-data-message">
      No changes found in the comparison.
    </div>
  </div>
</template>

<script>
export default {
  name: 'ComparisonTable',
  props: {
    comparisonData: {
      type: Object,
      required: true
    }
  },
  data() {
    return {
      error: null,
      processedData: [],
      tableHeaders: []
    }
  },
  computed: {
    hasData() {
      return this.processedData.length > 0;
    }
  },
  watch: {
    comparisonData: {
      immediate: true,
      handler(newData) {
        this.processData(newData);
      }
    }
  },
  methods: {
    processData(data) {
      this.error = null;
      this.processedData = [];
      this.tableHeaders = [];

      try {
        if (!data || typeof data !== 'object') {
          throw new Error('Invalid comparison data');
        }

        let rows = data.changed_rows || data;
        if (!Array.isArray(rows)) {
          rows = [rows];
        }

        this.processedData = rows.map(row => {
          if (typeof row !== 'object') return {};
          
          if (row.changes && typeof row.changes === 'object') {
            return { id: row.id, ...row.changes };
          }
          
          return row;
        });

        if (this.processedData.length > 0) {
          this.tableHeaders = Object.keys(this.processedData[0]);
        }

      } catch (err) {
        console.error('Error processing comparison data:', err);
        this.error = `Error processing comparison data: ${err.message}`;
      }
    }
  }
}
</script>

<style scoped>
.comparison-table {
  font-family: var(--font-family);
  color: var(--text-color);
  background-color: var(--surface-0);
  border-radius: var(--border-radius);
  box-shadow: var(--card-shadow);
  overflow: hidden;
}

.comparison-table table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0;
}

.comparison-table th, .comparison-table td {
  padding: 1rem;
  text-align: left;
  border-bottom: 1px solid var(--surface-border);
}

.comparison-table th {
  background-color: var(--surface-100);
  font-weight: 600;
  color: var(--surface-900);
}

.comparison-table tr:nth-child(even) {
  background-color: var(--surface-50);
}

.comparison-table tr:hover {
  background-color: var(--surface-100);
}

.old-value {
  color: var(--red-600);
  text-decoration: line-through;
  margin-right: 0.5rem;
}

.new-value {
  color: var(--green-600);
  margin-left: 0.5rem;
}

.arrow {
  color: var(--text-color-secondary);
}

.error-message, .no-data-message {
  padding: 1rem;
  border-radius: var(--border-radius);
  font-weight: 600;
}

.error-message {
  background-color: var(--red-100);
  color: var(--red-700);
  border: 1px solid var(--red-200);
}

.no-data-message {
  background-color: var(--surface-100);
  color: var(--text-color-secondary);
  border: 1px solid var(--surface-200);
}
</style>