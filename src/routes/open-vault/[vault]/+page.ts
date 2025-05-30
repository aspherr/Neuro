import type { PageLoad } from './$types';

// exports vault path
export const load: PageLoad = async ({ params }) => {
  const vaultPath = params.vault;

  if (!vaultPath) {
    throw new Error("Vault name is required");
  }
  return { vaultPath };
};
