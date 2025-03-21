<script lang="ts">
    import { onMount } from 'svelte';
    import { Window, LogicalSize, LogicalPosition, currentMonitor} from '@tauri-apps/api/window';

    const win = Window.getCurrent();

    onMount(async() => {
        await win.setSize(new LogicalSize(1400, 1000));
    });

    export let data: {
        vaultPath: string;
    };
    const decodedPath = data.vaultPath;

</script>

<main class="h-screen w-screen bg-zinc-900">
    <p class="text-white p-2">{decodedPath}</p>
</main>
