<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let appVersion = 'loading...';
  onMount(async() => {
    appVersion = await invoke ('get_app_version');
  });

  function createVault() {
    const message = "vault created";
    invoke("logger", { message });
  }

  function openVault() {
    const message = "vault opened";
    invoke("logger", { message });
  }

  function openSyncedVault() {
    const message = "synced vault opened";
    invoke("logger", { message });
  }
</script>


<main class="h-screen w-screen bg-zinc-900 text-white">
  <div class="">
    <img src="/logo.png" alt="Neuro Logo" class="w-42 h-48 mx-auto pt-6"/>
  </div>

  <div class="text-center">
    <h1 class="Satoshi font-bold text-5xl -mt-7">Neuro</h1>
    <p class="Satoshi text-base text-gray-400">Version: {appVersion}</p>
  </div>

  <div class="mt-11 w-full flex flex-col items-center">
    <div class="Satoshi font-base text-lg w-64 flex flex-col space-y-3">
      
      <button class="w-full py-3 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition antialiased" onclick={createVault}>
        Create Vault
      </button>
      
      <button class="w-full py-3 bg-gray-700 text-white rounded-lg hover:bg-gray-800 transition" onclick={openVault}>
        Open Vault
      </button>

      <button class="w-full py-3 bg-gray-700 text-white rounded-lg hover:bg-gray-800 transition" onclick={openSyncedVault}>
        Open Synced Vault
      </button>
    </div>
  
  </div>

</main>
