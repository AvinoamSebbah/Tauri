<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

// Refs pour les données utilisateur et la liste des utilisateurs
const user = ref<{ username: string, sid: string } | null>(null);
const users = ref<Array<{ username: string, sid: string }>>([]);

onMounted(async () => {
  try {
    user.value = await invoke('get_current_user');
    console.log('User:', user.value);
    
    users.value = await invoke('get_users_list');
  } catch (error) {
    console.error('Error fetching user data:', error);
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
        
        <!-- Message de bienvenue -->
        <v-card class="mb-5 pa-5 text-center">
          <h1>Welcome, {{ user?.username || 'Utilisateur' }} !</h1>
          <p>Your SID : <strong>{{ user?.sid || 'Inconnu' }}</strong></p>
          <p>Welcome to Norelbot</p>
        </v-card>

        <!-- Liste des utilisateurs -->
        <v-card class="pa-5">
          <h2 class="mb-4">Users list :</h2>
          <v-list>
            <v-list-item-group>
              <v-list-item
                v-for="(u, index) in users"
                :key="index"
                class="mb-2"
              >
                <v-list-item-content>
                  <v-list-item-title>{{ u.username }}</v-list-item-title>
                  <v-list-item-subtitle>SID: {{ u.sid }}</v-list-item-subtitle>
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
