<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import csvLoader from './modules/csv/csvLoader.vue';
import tobleronImage from './assets/tobleron.png';

// Refs for user data and the list of users
const user = ref<{ first_name: string,last_name: string, id: string, username: string } | null>(null);
const fetchError = ref<boolean>(false); // New ref to handle fetching errors
const usersList = ref<{ first_name: string, last_name: string, id: string, username: string }[]>([]); // List of users

const registerToWaitingList = async () => {
  try {
    await invoke('register_to_waiting_list');
    alert('Registration request sent!');
  } catch (error) {
    console.error('Error registering to the waiting list:', error);
  }
};

const validateUser = async (userId: string) => {
  try {
    await invoke('validate_user', { userId });
    usersList.value = usersList.value.filter((user) => user.id !== userId);
  } catch (error) {
    console.error(`Error validating user ${userId}:`, error);
  }
};

onMounted(async () => {
  try {
    // Fetch the current user
    user.value = await invoke('get_current_user');
    console.log('User:', user.value);

    // Fetch the list of users (if applicable)
    usersList.value = await invoke('get_users_waiting_list');
  } catch (error) {
    console.error('Error fetching user data or user list:', error);
    fetchError.value = true; // Trigger the error if fetching fails
  }
});
</script>

<template>
  <v-app>
    <v-main class="bg-grey-lighten-4">
      <v-container class="mx-auto" style="max-width: 768px">
        
        <div class="text-center my-5">
          <v-img :src="tobleronImage" max-width="150" class="mx-auto"></v-img>
        </div>

        <!-- Check for fetching error -->
        <v-card class="mb-5 pa-5 text-center">
          <template v-if="fetchError">
            <h2>Error fetching data</h2>
            <p>Please submit a registration request.</p>
            <v-btn color="primary" @click="registerToWaitingList">
              Register to waiting list
            </v-btn>
          </template>
          <template v-else>
            <!-- Welcome message -->
            <h1>Welcome, {{ user?.first_name || user?.username || 'User' }}!</h1>
            <p>Your ID: <strong>{{ user?.id || 'Unknown' }}</strong></p>
            <p>Welcome to Norelbot</p>
            
            <!-- List of users with validate button -->
            <v-list>
              <v-list-item-group>
                <v-list-item v-for="(userItem, index) in usersList" :key="index">
                  <v-list-item-content>
                    <v-list-item-title>
                      {{ userItem.first_name }} {{ userItem.last_name }} ({{ userItem.username }})
                    </v-list-item-title>
                    <v-list-item-subtitle>
                      ID: {{ userItem.id }}
                    </v-list-item-subtitle>
                  </v-list-item-content>
                  <!-- Validate button next to each user -->
                  <v-list-item-action>
                    <v-btn color="success" @click="validateUser(userItem.id)">
                      Validate
                    </v-btn>
                  </v-list-item-action>
                </v-list-item>
              </v-list-item-group>
            </v-list>
          </template>
        </v-card>
        <v-card class="mb-5 pa-5">
          <csv-loader></csv-loader>
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
