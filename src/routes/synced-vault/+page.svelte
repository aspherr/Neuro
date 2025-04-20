<script lang="ts">
    import { onMount } from 'svelte';
    import { Window, LogicalSize } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/core';
    import toast, {Toaster} from 'svelte-5-french-toast'

    const win = Window.getCurrent();

    onMount(async() => {
        await win.setSize(new LogicalSize(800, 800));
        await win.center();
    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <Toaster/>
  
    <div class="p-6 border-b border-zinc-800 flex items-center justify-between">
      <h1 class="text-3xl font-bold">XYZ's Vaults</h1>
    </div>

  </main>