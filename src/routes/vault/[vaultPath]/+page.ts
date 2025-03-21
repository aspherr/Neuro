import type { Load } from '@sveltejs/kit';

export const load: Load = ({ params }) => {
    if (!params.vaultPath) {
        throw new Error("Vault path is required");
      }
      const vaultPath = decodeURIComponent(params.vaultPath);
    
      return { vaultPath };
  };
  
