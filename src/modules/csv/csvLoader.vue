<template>
    <div>
      <h1>CSV Manager</h1>
      <v-file-input accept=".csv" @change="uploadCSV" label="Upload CSV" prepend-icon="mdi-upload" />
  
      <table>
        <thead>
          <tr>
            <th v-for="(header, index) in tableHeaders" :key="index">{{ header }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, index) in tableData" :key="index">
            <td v-for="(value, colIndex) in row" :key="colIndex">{{ value }}</td>
          </tr>
        </tbody>
      </table>
      <v-btn @click="downloadCSV" color="primary" class="mt-4">Download CSV</v-btn>
     <v-btn @click="clearMeasureTable" color="red" dark class="mt-4 ml-5">
      <v-icon left>mdi-delete</v-icon>
    </v-btn>
    </div>
  </template>
  
  <script lang="ts" setup>
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api';
  
  const tableHeaders = ref<string[]>([]);
  const tableData = ref<any[]>([]);
  
const clearMeasureTable = async () => {
  try {
    await invoke('clear_measure_table');
    console.log('Measure table cleared successfully.');
    tableData.value = [];
  } catch (error) {
    console.error('Failed to clear measure table', error);
  }
};

  const arrayToCSV = (data: any[]): string => {
  if (data.length === 0) return '';

  const headers = Object.keys(data[0]);
  const csvRows = [
    headers.join(';'),
    ...data.map((row) => headers.map((header) => row[header]).join(';')),
  ];

  return csvRows.join('\n'); 
};
  
  const downloadCSV = async () => {
    try {
      const data = await invoke<any[]>('get_table_data');
      if (data.length) {
        const csvData = arrayToCSV(data);
        const blob = new Blob([csvData], { type: 'text/csv;charset=utf-8;' });
        const url = window.URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.setAttribute('download', 'data.csv');
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
      } else {
        console.warn('No data available to download.');
      }
    } catch (error) {
      console.error('Failed to download CSV', error);
    }
  };

const csvToArray = (csvContent: string): any[] => {
  const lines = csvContent.trim().split('\n');
  
  const headers = lines[0].replace('\r', '').split(';');
  
  const rows = lines.slice(1).map(line => {
    const values = line.replace('\r', '').split(';');
    const obj: Record<string, string | number> = {};
    
    headers.forEach((header, index) => {
      obj[header] = isNaN(Number(values[index])) ? values[index] : Number(values[index]);
    });

    return obj;
  });

  return rows;
};

  
  const uploadCSV = async (event: Event) => {
    const file = (event.target as HTMLInputElement).files?.[0];
    if (!file) return;
  
    const reader = new FileReader();
    reader.onload = async (e: ProgressEvent<FileReader>) => {
      const content = e.target?.result as string;
      const parsedData = csvToArray(content);
  
      try {
        console.log('Uploading CSV data:', parsedData);
        const result = await invoke('add_data_to_table', { data: parsedData }) as any; 
          console.log('CSV successfully uploaded and data added.');
          loadTableData();
      } catch (error) {
        console.error('Failed to upload CSV', error);
      }
    };
  
    reader.readAsText(file);
  };
  
  const loadTableData = async () => {
    try {
      const data = await invoke<any[]>('get_table_data');
      if (data.length) {
        tableHeaders.value = Object.keys(data[0]);
        tableData.value = data;
      }
    } catch (error) {
      console.error('Failed to load table data', error);
    }
  };
  
  loadTableData();
  </script>
  
  <style scoped>
  table {
    width: 100%;
    border-collapse: collapse;
  }
  th, td {
    border: 1px solid #ddd;
    padding: 8px;
  }
  th {
    background-color: #f2f2f2;
  }
  </style>
  