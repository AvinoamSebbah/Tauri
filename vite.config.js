import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import vuetify from 'vite-plugin-vuetify';
import { fileURLToPath, URL } from 'node:url';
import { PrimeVueResolver } from '@primevue/auto-import-resolver';
import Components from 'unplugin-vue-components/vite';

export default defineConfig({
  clearScreen: false,
  server: {
    port: 8080,
    strictPort: true,
  },
  optimizeDeps: {
    noDiscovery: true
},
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome97', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  plugins: [
    vue(),
    vuetify({
      autoImport: true,
    }),
    Components({
      resolvers: [PrimeVueResolver()]
  })
  ],
  resolve: {
    alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url))
    }
}
});
