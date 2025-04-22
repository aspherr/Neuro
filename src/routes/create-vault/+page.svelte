<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { fade, slide } from "svelte/transition";
    import { open } from '@tauri-apps/plugin-dialog';
    import { mkdir } from '@tauri-apps/plugin-fs';
    import { goto } from "$app/navigation";
  
    let name = "";
    let path = "";
    let appVersion = 'loading...';

    // On initial page load
    onMount(async() => {
        appVersion = await invoke ('get_app_version');
    });

    // opens dialog for path selection
    const openFinder = async () => {
        try {
        const selectedPath = await open({
            directory: true,
            multiple: false
        });

        if (selectedPath) {
            console.log(selectedPath);
            path = selectedPath
        }

        } catch (err) {
            console.error(err);
            alert(err);
        }
    }
    
    // Creates vault path in user's file system
    async function createVault() {
      if (!name || !path) {
        alert("Please enter a vault name and choose a location.");
        return;
      }

      try {
        const vaultPath = `${path}/${name}`;
        await mkdir(vaultPath, { recursive: true });
        alert("Vault created successfully");
        
        if (vaultPath) {
            const encodedPath = encodeURIComponent(vaultPath);
            goto(`/open-vault/${encodedPath}`);
        }

      } catch (err) {
        console.error(err);
        alert(err);
      }
      
    }
    
    // Redirects back to the index page
    function goBack() {
        goto("/");
    }
  </script>
  
  <main class="h-screen w-screen bg-zinc-900 text-white flex flex-col items-center justify-center space-y-6"> 
    <!-- Title -->  
    <div out:fade={{ duration: 150 }} class="text-center">
        <h1 class="Satoshi font-bold text-5xl -mt-7">Neuro</h1>
        <p class="Satoshi text-base text-gray-400">Version: {appVersion}</p>
    </div>

    <div in:slide out:fade={{ duration: 150 }} class="w-full max-w-lg p-6">

        <!-- Back button that redirects back to the index page -->
        <button class="text-gray-400 hover:text-white text-lg flex items-center space-x-2 mb-4" onclick={goBack}>
            ← <span class="pl-2"> Back</span>
        </button>

        <h2 class="text-xl font-bold mb-4">Create local vault</h2>

        <!-- Vault name input -->
        <input type="text" bind:value={name} placeholder="Vault name"
        class="w-full bg-zinc-700 text-white p-3 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-600 mb-4"/>

        <!-- Open dialog button -->
        <div class="flex gap-2">
            <input type="text" bind:value={path} placeholder="Choose location..." disabled
                    class="flex-1 bg-zinc-700 text-gray-400 p-3 rounded-md"/>
            <button class="bg-gray-700 text-white px-4 py-2 rounded-md hover:bg-gray-600" onclick={openFinder}>Browse</button>
        </div>

        <!-- Submit button for create vault form -->
        <button onclick={createVault} 
                class="w-full mt-6 bg-orange-700 text-white font-semibold py-3 rounded-md hover:bg-orange-600 transition">
            Create
        </button>
    </div>
</main>
