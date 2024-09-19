/// <reference types="vite/client" />

declare interface ImportMeta {
  readonly env: ImportMetaEnv;
}

interface ImportMetaEnv {
  readonly VITE_NETWORK_PROCESSOR: string;
  // Ajoute ici d'autres variables d'environnement si nécessaire
  readonly VITE_API_URL: string;
  // ...
}
