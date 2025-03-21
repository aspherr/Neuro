<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { fade, slide } from "svelte/transition";
    import { open } from '@tauri-apps/plugin-dialog';
    import { writeFile, mkdir } from '@tauri-apps/plugin-fs';
    import { goto } from "$app/navigation";
  
    let name = "";
    let path = "";
    let appVersion = 'loading...';

    onMount(async() => {
        appVersion = await invoke ('get_app_version');
    });

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
  
    async function createVault() {
      if (!name || !path) {
        alert("Please enter a vault name and choose a location.");
        return;
      }
      console.log("Creating Vault:", { name, path });

      try {
        const vaultPath = `${path}/${name}`;
        await mkdir(vaultPath, { recursive: true });

        const fileContent = new TextEncoder().encode(`# Welcome to ${name}`);
        await writeFile(`${vaultPath}/README.md`, fileContent);

        alert("Vault created successfully");
        if (vaultPath) {
            const encodedPath = encodeURIComponent(vaultPath);
            goto(`/vault/${encodedPath}`);
        }

      } catch (err) {
        console.error(err);
        alert(err);
      }
      
    }
  
    function goBack() {
        goto("/");
    }
  </script>
  
  <main class="h-screen w-screen bg-zinc-900 text-white flex flex-col items-center justify-center space-y-6">    
    <div out:fade={{ duration: 150 }} class="text-center">
        <h1 class="Satoshi font-bold text-5xl -mt-7">Neuro</h1>
        <p class="Satoshi text-base text-gray-400">Version: {appVersion}</p>
    </div>

    <div in:slide out:fade={{ duration: 150 }} class="w-full max-w-lg p-6">

        <button class="text-gray-400 hover:text-white text-lg flex items-center space-x-2 mb-4" onclick={goBack}>
            ← <span class="pl-2"> Back</span>
        </button>

        <h2 class="text-xl font-bold mb-4">Create local vault</h2>

        <input type="text" bind:value={name} placeholder="Vault name"
        class="w-full bg-zinc-700 text-white p-3 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-600 mb-4"/>

        <div class="flex gap-2">
            <input type="text" bind:value={path} placeholder="Choose location..." disabled
                    class="flex-1 bg-zinc-700 text-gray-400 p-3 rounded-md"/>
            <button class="bg-gray-700 text-white px-4 py-2 rounded-md hover:bg-gray-600" onclick={openFinder}>Browse</button>
        </div>

        <button onclick={createVault} 
                class="w-full mt-6 bg-orange-700 text-white font-semibold py-3 rounded-md hover:bg-orange-600 transition">
            Create
        </button>
    </div>
</main>
