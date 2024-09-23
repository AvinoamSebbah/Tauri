<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

// Refs for user data and the list of users
const user = ref<{ first_name: string,last_name: string, id: string, username: string } | null>(null);
const users = ref<Array<{ username: string, id: string }>>([]);
const fetchError = ref<boolean>(false); // New ref to handle fetching errors

const registerToWaitingList = async () => {
  try {
    await invoke('register_to_waiting_list');
    alert('Registration request sent!');
  } catch (error) {
    console.error('Error registering to the waiting list:', error);
  }
};

onMounted(async () => {
  try {
    user.value = await invoke('get_current_user');
    users.value = await invoke('get_users_waiting_list');
    console.log('Users:', users.value);
    console.log('User:', user.value);
  } catch (error) {
    console.error('Error fetching user data:', error);
    fetchError.value = true; // Trigger the error if fetching fails
  }
});
</script>

<template>
  <v-app>
    <v-main class="bg-grey-lighten-4">
      <v-container class="mx-auto" style="max-width: 768px">
        
        <div class="text-center my-5">
          <v-img src="/src/assets/tobleron.png" max-width="150" class="mx-auto"></v-img>
        </div>

        <!-- Check for fetching error -->
        <v-card class="mb-5 pa-5 text-center">
          <template v-if="fetchError">
            <h2>Welcome new User</h2>
            <p>Please submit a registration request.</p>
            <v-btn color="primary" @click="registerToWaitingList" class="mt-3">
              Register to waiting list
            </v-btn>
          </template>
          <template v-else>
            <h1>Welcome, {{ user?.first_name || user?.username || 'User' }}!</h1>
            <p>Your ID: <strong>{{ user?.id || 'Unknown' }}</strong></p>
            <p>Welcome to Norelbot</p>
          </template>
        </v-card>
        <v-card class="pa-5">
          <h2 class="mb-4">Users waiting list :</h2>
          <v-list>
            <v-list-item-group>
              <v-list-item
                v-for="(u, index) in users"
                :key="index"
                class="mb-2"
              >
                <v-list-item-content>
                  <v-list-item-title>{{ u.username }}</v-list-item-title>
                  <v-list-item-subtitle>ID: {{ u.id }}</v-list-item-subtitle>
                </v-list-item-content>
              </v-list-item>
            </v-list-item-group>
          </v-list>
        </v-card>
      </v-container>
    </v-main>
  </v-app>
</template>

<style scoped>
h1 {
  font-size: 2rem;
  font-weight: bold;
}
p {
  font-size: 1.2rem;
}
</style>
