<script lang="ts">
    import { onMount } from 'svelte';
    import { Window, LogicalSize } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/core';
    import toast, {Toaster} from 'svelte-5-french-toast'

    const win = Window.getCurrent();
    let session_token = localStorage.getItem("session_token");
    
    interface User {
      forename: string;
      email: string;
    }
    let account: User;
    
    async function get_user_data() {
      account = await invoke('get_user_data', { sessionToken: session_token })
    }

    onMount(async() => {
        await win.setSize(new LogicalSize(800, 800));
        await win.center();
        get_user_data();
    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <Toaster/>
  
    <div class="p-6 border-b border-zinc-800 flex items-center justify-between">
      {#if account}
        <h1 class="text-3xl font-bold">{account.forename}'s Vaults</h1>
      {/if}
    </div>

  </main> 