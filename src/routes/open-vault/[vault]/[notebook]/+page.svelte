<script lang="ts">
    import { onMount } from 'svelte';
    import { fly } from 'svelte/transition';
    import { goto } from "$app/navigation";

    let toggle = true;

    export let data: {
        vaultPath: string;
    };
    const notebookName = data.vaultPath.split('/').filter(Boolean).pop() || '';

    onMount(async() => {

    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <div class="h-screen w-65 absolute bg-zinc-800 transistion duration-400 ease-in-out z-0"
        class:w-0={!toggle} 
        class:w-65={toggle}>

        <div class="group bg-zinc-800 w-9 h-9 p-1 mt-7 ml-7 rounded hover:bg-zinc-700 transistion-colors duration-200 antialiased z-1">
            <button class="text-white" aria-label="toggle-button" on:click={() => toggle = !toggle}>
                <svg
                  class="w-7 h-7 group-hover:text-orange-500 transistion-colors duration-200"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24"
                >
                  <path d="M4 6h16M4 12h16M4 18h16"/>
                </svg>
              </button>
        </div>

        <div class="">
            {#if toggle}
            <h1 class="absolute font-semibold pl-8 pt-8 antialiased z-1" class:whitespace-wrap={!toggle} transition:fly={{ x: -100 }}>
                {notebookName}
            </h1>
            {/if}
        </div>    
    </div>
</main>