<script lang="ts">
    import { onMount } from 'svelte';
    import { Window, LogicalSize } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/core';
    import toast, {Toaster} from 'svelte-5-french-toast'
    import Button from '../../components/button.svelte';
    import { goto } from '$app/navigation';
    import { ask } from '@tauri-apps/plugin-dialog';

    const win = Window.getCurrent();
    let session_token = localStorage.getItem("session_token");
    let createModal = false;
    let vaultName = "";
    let names: string[] = [];
    
    // user account structure
    interface User {
      forename: string;
      email: string;
    }
    let account: User;
    let user_id = "";
    
    // Gets user's data from active session
    async function get_user_data() {
      account = await invoke('get_user_data', { sessionToken: session_token })
      user_id = await invoke('get_id', {email: account.email})
      loadVaults();
    }

    // Loads in remote vaults
    async function loadVaults() {
      names = await invoke('get_vault_names', {id: user_id});
    }

    // Creates a remote vault
    async function create_vault() {
      await invoke('add_vault', {
        name: vaultName,
        id: user_id
      });

      vaultName = "";
      createModal = false;
      loadVaults();
      toast.success('Vault Created!');
    }

    // redirects to dynamic vault page
    async function open_vault(vault: string) {
      goto(`../open-vault/${vault}`);
    }

    // deletes remote vault
    async function delete_vault(name: string) {
        // asks user for confirmation before deletion
        const confirmDelete = await ask('This action cannot be reverted. Are you sure?', {
        title: 'Delete Note',
        kind: 'warning',
        });

        if (confirmDelete) {
          let vault_id = await invoke<string>('vault_id', { name: name }); // vault ID
          await invoke<string>('drop_vault', { 
            vid: vault_id,
            name: name,
            uid: user_id
           }); 
          toast.success('Vault Deleted!');
          loadVaults();
        }
    }

    // Logout  the user and delete sesssion data
    async function logout() {
      let logout = await invoke('logout', {
        token: session_token,
        id: user_id
      });

      if (logout) {
        localStorage.removeItem("session_token");
        await win.setSize(new LogicalSize(800, 650));
        await win.center();
        goto("/")
      }
    }

    //  On initial page load
    onMount(async() => {
        await win.setSize(new LogicalSize(800, 800));
        await win.center();
        get_user_data();
    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <Toaster/>
  
    <div class="p-6 border-b border-zinc-800 flex items-center justify-between">
      <!-- Title -->
      {#if account}
        <h1 class="text-3xl font-bold">{account.forename}'s Vaults</h1>
      {/if}

      <!-- Create button that toggles a modal -->
      <div class="flex gap-4">
        <Button clickEvent={() => createModal = true}>
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


     <!-- Create vault modal -->
    {#if createModal}
    <div class="fixed inset-0 backdrop-blur-md flex items-center justify-center z-50">
      <div class="bg-zinc-800 text-white p-6 rounded-lg w-full max-w-md shadow-lg">
        <h2 class="text-xl font-bold mb-4">Create a New Notebook</h2>  <!-- Title -->

         <!-- User Input -->
        <input
          type="text"
          bind:value={vaultName}
          placeholder="Notebook name"
          class="w-full p-3 rounded-md bg-zinc-700 text-white focus:outline-none focus:ring-2 focus:ring-orange-600"
        />

         <!-- Cancel button -->
        <div class="flex justify-end gap-2 mt-6">
          <button class="px-4 py-2 bg-zinc-600 rounded-md hover:bg-zinc-700 transition"
            on:click={() => createModal = false}>
            Cancel
          </button>

           <!-- Create vault button -->
          <button
            class="px-4 py-2 bg-orange-700 rounded-md hover:bg-orange-600 transition"
            on:click={create_vault}>
            Create
          </button>
        </div>
      </div>
    </div>
    {/if}

    <div class="flex-1 p-6 overflow-auto">
       <!-- Display message if no vaults found -->
      {#if names.length === 0}
        <div class="pt-2 text-gray-400">
          <p class="mb-4">You haven't created any notebooks yet.</p>
        </div>
    
      {:else}
      <div class="grid grid-cols-2 md:grid-cols-2 gap-4">
         <!-- Loop through all vaults and display them as cards -->
        {#each names as vaultName}
        <div class="relative group">

           <!-- Redirects user to that notebook if clicked -->
          <button class="p-1 py-10 w-full min-h-[250px] bg-zinc-800 hover:bg-zinc-700 transition cursor-pointer border-t-5 border-orange-600 rounded"
          on:click={() => open_vault(vaultName)}>
            <h2 class="font-bold text-lg">{vaultName}</h2>
          </button>

           <!-- Trash icon to delete the notebook -->
          <button class="absolute top-3 right-2  opacity-0 group-hover:opacity-100" aria-label="delete-button"
            on:click={() => {delete_vault(vaultName)}}>
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

    <div class="flex items-center gap-2 px-6 py-4 border-t border-zinc-800 text-gray-500 text-sm">
       <!-- Logout button -->
      <Button clickEvent={logout}>
        <svg 
        class="w-5 h-5 group-hover:text-orange-600 transform transition-transform duration-200 ease-in-out"
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 5l-7 7 7 7" />
        </svg>
      </Button>
    </div>

  </main> 