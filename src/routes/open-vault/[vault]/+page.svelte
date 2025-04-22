<script lang="ts">
    import { onMount } from 'svelte';
    import { Window, LogicalSize } from '@tauri-apps/api/window';
    import { readDir } from '@tauri-apps/plugin-fs';
    import { mkdir } from '@tauri-apps/plugin-fs';
    import { goto } from "$app/navigation";
    import { ask } from '@tauri-apps/plugin-dialog'; 
    import { invoke } from '@tauri-apps/api/core';
    import toast, {Toaster} from 'svelte-5-french-toast'
    import Button from '../../../components/button.svelte';

    const win = Window.getCurrent();
    let showModal = false;
    let notebookName = '';
    $: session_token = localStorage.getItem("session_token");

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

    type RemoteNotebookEntry = {
        name: string;
    };

    let notebooks: NotebookEntry[] = [];
    let remoteNotebooks: RemoteNotebookEntry[] = [];

    $: notebookPanes = session_token ? remoteNotebooks : notebooks;
  
    interface User {
      forename: string;
      email: string;
    }
    let account: User;
    let user_id = "";

    async function createNotebook() {
      if (!notebookName.trim()) {
        alert("Please enter a notebook name.");
        return;
      }

      if (session_token && session_token !== "null" && session_token !== "undefined") {
        await invoke('add_notebook', {
          name: notebookName,
          id: user_id
        })

      } else {
        const notebookPath = `${decodedPath}/${notebookName}`;
        await invoke<string>('create_folder', { path: notebookPath });
      }

      toast.success('Notebook Created!');

      await loadNotebooks();
        
      showModal = false;
      notebookName = '';
    }

    async function loadNotebooks() {
      if (session_token && session_token !== "null" && session_token !== "undefined") {
        remoteNotebooks = (await invoke<string[]>('get_notebook_names', { id: user_id }))
        .map(name => ({ name }));
        return;
      }
      
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
        await win.setSize(new LogicalSize(800, 650));
        await win.center();
        goto("/");
    }

    function openNotebook(notebook: string | undefined) {
      if (!notebook) {
        return;
      };
      
      const encodedPath = encodeURIComponent(decodedPath);
      goto(`${encodedPath}/${notebook}`);
    }

    async function deleteNotebook(notebook: string | undefined) {
      const confirmDelete = await ask('This action cannot be reverted. Are you sure?', {
        title: 'Delete Notebook',
        kind: 'warning',
        });
      
      if (confirmDelete) {
        let notebookPath = `${decodedPath}/${notebook}`;
        await invoke<string>('delete_folder', { path: notebookPath });
        toast.success('Notebook Deleted!')
      }
      loadNotebooks()
    }

    onMount(async() => {
        await win.setSize(new LogicalSize(1400, 1000));
        await win.center();
        if (session_token && session_token !== "null" && session_token !== "undefined") {
        account = await invoke('get_user_data', { sessionToken: session_token });
        user_id = await invoke('get_id', {email: account.email});
        }
        await loadNotebooks();
    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <Toaster/>
  
    <!-- Header -->
    <div class="p-6 border-b border-zinc-800 flex items-center justify-between">
      <h1 class="text-3xl font-bold">Your Notebooks</h1>

      <div class="flex gap-4">
        <Button clickEvent={() => showModal = true}>
          <svg xmlns="http://www.w3.org/2000/svg"
          class="group-hover:text-orange-500 transistion-colors duration-200"
          width="24" height="24"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          stroke-width="1.5">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
          </svg>
        </Button>
      </div>
    
    </div>

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

    <!-- Main content -->
    <div class="flex-1 p-6 overflow-auto">
      {#if notebookPanes.length === 0}
        <div class="pt-2 text-gray-400">
          <p class="mb-4">You haven't created any notebooks yet.</p>
        </div>
      
      {:else}
      <div class="grid grid-cols-2 md:grid-cols-6 gap-4">
        {#each notebookPanes as notebook (notebook.name)}
        <div class="relative group">
          <button class="p-4 py-6 w-full min-h-[250px] bg-zinc-800 hover:bg-zinc-700 transition cursor-pointer border-l-5 border-orange-600 rounded"
            on:click={() => openNotebook(notebook.name)}>
            <h2 class="font-bold text-lg">{notebook.name}</h2>
          </button>

          <button class="absolute top-2 right-2  opacity-0 group-hover:opacity-100" aria-label="delete-button" on:click={() => {deleteNotebook(notebook.name)}}>
              <svg xmlns="http://www.w3.org/2000/svg"
              class="text-gray-400 hover:text-red-500"
              viewBox="0 0 24 24" 
              width="24" height="24" 
              fill="none" stroke="currentColor" 
              stroke-width="1.5" 
              stroke-linecap="round" stroke-linejoin="round">
                  <path d="M3 6h18" />
                  <path d="M8 6V4h8v2" />
                  <path d="M19 6l-1 14H6L5 6" />
                  <line x1="10" y1="11" x2="10" y2="17" />
                  <line x1="14" y1="11" x2="14" y2="17" />
              </svg>  
          </button>
        </div>
        {/each}
      </div>
      {/if}
    </div>
  
    <!-- Footer with vault path -->
    <div class="flex items-center gap-2 px-6 py-4 border-t border-zinc-800 text-gray-500 text-sm">
      <Button clickEvent={goBack}>
        <svg 
        class="w-5 h-5 group-hover:text-orange-600 transform transition-transform duration-200 ease-in-out"
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 5l-7 7 7 7" />
        </svg>
      </Button>

      <span class="pl-3">
        Vault location: <span class="text-gray-300">{decodedPath}</span>
      </span>
    </div>
  </main>
