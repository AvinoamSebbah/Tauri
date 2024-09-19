<script setup lang="ts">
import { invoke, defineComponent } from '@tauri-apps/api/tauri';
import { ref, onMounted } from 'vue';
/* template specific */
const snackbarVisibility = ref(false);
import Counter from './modules/counter/mod.vue';
import Fetcher from './modules/fetcher/mod.vue';


const newUserName = ref('');
const users = ref([]);

// Fonction pour ajouter un utilisateur
const addUser = async () => {
  if (newUserName.value.trim()) {
    await invoke('insert_user', { name: newUserName.value });
    newUserName.value = '';
    loadUsers(); // Recharger la liste des utilisateurs après ajout
  }
};

// Fonction pour récupérer les utilisateurs
const loadUsers = async () => {
  users.value = await invoke('get_users');
  users.value.forEach(user => user.newName = user.name); // Préparer le champ pour la mise à jour
};

// Fonction pour mettre à jour un utilisateur
const updateUser = async (id, newName) => {
  if (newName.trim()) {
    await invoke('update_user', { id, name: newName });
    loadUsers(); // Recharger la liste après modification
  }
};

// Fonction pour supprimer un utilisateur
const deleteUser = async (id) => {
  await invoke('delete_user', { id });
  loadUsers(); // Recharger la liste après suppression
};

// Charger les utilisateurs au démarrage du composant
onMounted(() => {
  loadUsers();
});

</script>

<style scoped>
.container {
  margin: 0 auto;
  max-width: 600px;
  padding: 20px;
  text-align: center;
}

input {
  margin: 10px;
  padding: 5px;
}

button {
  margin: 5px;
  padding: 5px 10px;
  cursor: pointer;
}

ul {
  list-style: none;
  padding: 0;
}

li {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 5px 0;
}
</style>

<template>
  <v-app>
    <v-main class="bg-grey-lighten-4">
      <v-container class="mx-auto" style="max-width: 768px">
      
        

        <h1>Gestion des utilisateurs</h1>

<!-- Ajouter un utilisateur -->
<div>
  <input v-model="newUserName" placeholder="Nom de l'utilisateur" />
  <button @click="addUser">Ajouter un utilisateur</button>
</div>

<!-- Liste des utilisateurs -->
<div v-if="users.length > 0">
  <h2>Liste des utilisateurs</h2>
  <ul>
    <li v-for="user in users" :key="user.id">
      <span>{{ user.name }}</span>
      <input v-model="user.newName" placeholder="Nouveau nom" />
      <button @click="updateUser(user.id, user.newName)">Modifier</button>
      <button @click="deleteUser(user.id)">Supprimer</button>
    </li>
  </ul>
</div>

<!-- Afficher un message si aucun utilisateur -->
<div v-else>
  <p>Aucun utilisateur trouvé.</p>
</div>
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
