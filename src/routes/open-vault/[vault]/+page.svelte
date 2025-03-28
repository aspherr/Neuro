<script lang="ts">
    import { onMount } from 'svelte';
    import { Window, LogicalSize } from '@tauri-apps/api/window';
    import { readDir } from '@tauri-apps/plugin-fs';
    import { mkdir } from '@tauri-apps/plugin-fs';
    import { goto } from "$app/navigation";

    const win = Window.getCurrent();
    let showModal = false;
    let notebookName = '';

    export let data: {
        vaultPath: string;
    };
    const decodedPath = data.vaultPath;
  
    type NotebookEntry = {
        path: string;
        name?: string;
        children?: NotebookEntry[];
        isDir: boolean;
    };

    let notebooks: NotebookEntry[] = [];

    async function createNotebook() {
      if (!notebookName.trim()) {
        alert("Please enter a notebook name.");
        return;
      }

      try {
        const notebookPath = `${decodedPath}/${notebookName}`;
        await mkdir(notebookPath, { recursive: true });
        alert("Notebook created successfully");
        

      } catch (err) {
        console.error(err);
        alert(err);
      }

      showModal = false;
      notebookName = '';
    }

    async function loadNotebooks() {
      const result = await readDir(decodedPath);
      notebooks = (result as unknown as NotebookEntry[])
      .filter(entry => entry.name && !entry.name.startsWith('.'))
      .map(entry => ({
            path: entry.path,
            name: entry.name,
            isDir: entry.isDir,
        }));
    }

    async function goBack() {
        await win.setSize(new LogicalSize(800, 600));
        goto("/");
    }

    onMount(async() => {
        await win.setSize(new LogicalSize(1400, 1000));
        await loadNotebooks();
    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <!-- Header -->
    <div class="p-6 border-b border-zinc-800">
      <h1 class="text-3xl font-bold">Your Notebooks</h1>
    </div>
  
    <!-- Main content -->
    <div class="flex-1 p-6 overflow-auto">
      {#if notebooks.length === 0}
        <div class="pt-2 text-gray-400">
          <p class="mb-4">You haven't created any notebooks yet.</p>
            <button class="rounded-md px-6 py-2 bg-zinc-600 text-white font-semibold 
                          hover:bg-zinc-700 border border-transparent hover:border-orange-600 transition antialiased"
            on:click={() => showModal = true}>
              Create Notebook
            </button>

            {#if showModal}
              <div class="fixed inset-0 backdrop-blur-md flex items-center justify-center z-50">
                <div class="bg-zinc-800 text-white p-6 rounded-lg w-full max-w-md shadow-lg">
                  <h2 class="text-xl font-bold mb-4">Create a New Notebook</h2>

                  <input
                    type="text"
                    bind:value={notebookName}
                    placeholder="Notebook name"
                    class="w-full p-3 rounded-md bg-zinc-700 text-white focus:outline-none focus:ring-2 focus:ring-orange-600"
                  />

                  <div class="flex justify-end gap-2 mt-6">
                    <button class="px-4 py-2 bg-zinc-600 rounded-md hover:bg-zinc-700 transition"
                      on:click={() => showModal = false}>
                      Cancel
                    </button>

                    <button
                      class="px-4 py-2 bg-orange-700 rounded-md hover:bg-orange-600 transition"
                      on:click={createNotebook}>
                      Create
                    </button>
                  </div>
                </div>
              </div>
            {/if}

        </div>
      
      {:else}
        <div class="grid grid-cols-2 md:grid-cols-6 gap-4">
          {#each notebooks as notebook (notebook.name)}
            <div class="p-4 py-6 min-h-[200px] bg-zinc-800 hover:bg-zinc-700 transition cursor-pointer border-l-5 border-orange-600 rounded">
              <h2 class="font-bold text-lg">{notebook.name}</h2>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  
    <!-- Footer with vault path -->
    <div class="flex items-center gap-2 px-6 py-4 border-t border-zinc-800 text-gray-500 text-sm">
      <button aria-label='back-button' class="text-gray-400 hover:text-white text-base flex items-center pb-0.5"
      on:click={goBack}>
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </button>
  
    <span>
      Vault location: <span class="text-gray-300">{decodedPath}</span>
    </span>
  </main>
