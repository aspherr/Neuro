<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { open } from '@tauri-apps/plugin-dialog';

  let appVersion = 'loading...';

  //  On initial page load
  onMount(async() => {
    appVersion = await invoke ('get_app_version');
    localStorage.removeItem("session_token");
  });

  // Redirects to vault creation page
  function createVault() {
    goto("/create-vault");
  }

  // Opens vault url
  const openVault = async () => {
    try {
      const path = await open({
        directory: true,
        multiple: false
      });
      console.log(path);

      if (path) {
        const encodedPath = encodeURIComponent(path);
        goto(`/open-vault/${encodedPath}`);
      }

    } catch (err) {
      console.error(err);
    }
  }
  
  // Redirects to user authentication page
  function login() {
    goto('./user-auth/login');
  }
</script> 


<main class="h-screen w-screen bg-zinc-900 text-white">
  
  <!-- Title and logo -->
  <div>
    <img src="/logo.png" alt="Neuro Logo" class="w-42 h-48 mx-auto pt-6"/>
  </div>
  <div class="text-center">
    <h1 class="Satoshi font-bold text-5xl -mt-7">Neuro</h1>
    <p class="Satoshi text-base text-gray-400">Version: {appVersion}</p>
  </div>

  <div class="mt-11 w-full flex flex-col items-center">
    <div class="w-full max-w-md mx-auto mt-6 space-y-3">

        <!-- Create vault button -->
      <div class="flex justify-between items-center px-1 py-3 border-b border-gray-700">
          <div>
              <h3 class="text-white text-lg font-medium">Create new vault</h3>
              <p class="text-gray-400 text-sm">Create a new Neuro vault under a folder.</p>
          </div>
          <button class="w-24 h-10 bg-orange-700 border border-orange-500 text-white rounded-md hover:bg-orange-600 transition antialiased"
          on:click={createVault}>
              Create
          </button>
      </div>
  
      <!-- Open local vault button -->
      <div class="flex justify-between items-center px-1 py-3 border-b border-gray-700">
          <div>
              <h3 class="text-white text-lg font-medium">Open folder as vault</h3>
              <p class="text-gray-400 text-sm">Choose an existing folder of Markdown files.</p>
          </div>
          <button class="w-24 h-10 bg-gray-700 border border-gray-500 text-white rounded-md hover:bg-gray-600 transition"
          on:click={openVault}>
              Open
          </button>
      </div>

      <!-- Sign in to remote vault button -->
      <div class="flex justify-between items-center px-1 py-3 border-b border-gray-700">
        <div>
            <h3 class="text-white text-lg font-medium">Open vault from Neuro sync</h3>
            <p class="text-gray-400 text-sm">Set up a remote vault.</p>
        </div>
        <button class="w-24 h-10 bg-gray-700 border border-gray-500 text-white rounded-md hover:bg-gray-600 transition"
        on:click={login}>
            Sign In
        </button>
    </div>
  </div>
</main>
