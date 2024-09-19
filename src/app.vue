<script setup lang="ts">
import { ref } from 'vue';
/* template specific */
const snackbarVisibility = ref(false);
import Counter from './modules/counter/mod.vue';
import Fetcher from './modules/fetcher/mod.vue';
</script>

<template>
  <v-app>
    <v-main class="bg-grey-lighten-4">
      <v-container class="mx-auto" style="max-width: 768px">
        <v-btn @click="snackbarVisibility = !snackbarVisibility">
          Toggle Snackbar
        </v-btn>
        <counter />

        <fetcher class="mt-3" />
        <v-snackbar v-model="snackbarVisibility">
          Avinoam
          <template #actions>
            <v-btn icon="mdi-close" />
          </template>
        </v-snackbar>
      </v-container>
    </v-main>
  </v-app>
</template>
