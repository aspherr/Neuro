import type { PageLoad } from './$types';

// exports vault path and notebook name
export const load: PageLoad = async ({ params }) => {
  const path = params.vault;
  const name = params.notebook;

  if (!name || !path) {
    throw new Error("Invalid Routing");
  }
  return { name, path };
};
    