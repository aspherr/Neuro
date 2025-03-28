<script lang="ts">
    import { onMount } from 'svelte';
    import { Window, LogicalSize } from '@tauri-apps/api/window';
    import { readDir } from '@tauri-apps/plugin-fs';

    const win = Window.getCurrent();

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

    onMount(async() => {
        await win.setSize(new LogicalSize(1400, 1000));
        const result = await readDir(decodedPath);
        notebooks = (result as unknown as NotebookEntry[]).filter(entry =>
          entry.isDir && 
          entry.name && 
          !entry.name.startsWith('.')
        );
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
        <div class="text-center mt-12 text-gray-400">
          <p class="mb-4">You haven't created any notebooks yet.</p>
        </div>
      
      {:else}
        <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
          {#each notebooks as notebook (notebook.name)}
            <div class="p-4 bg-zinc-800 rounded-md hover:bg-zinc-700 transition cursor-pointer">
              <h2 class="font-bold text-lg">{notebook.name}</h2>
              <p class="text-sm text-gray-400">Click to open</p>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  
    <!-- Footer with vault path -->
    <div class="px-6 py-4 border-t border-zinc-800 text-gray-500 text-sm">
      Vault location: <span class="text-gray-300">{decodedPath}</span>
    </div>
  </main>
