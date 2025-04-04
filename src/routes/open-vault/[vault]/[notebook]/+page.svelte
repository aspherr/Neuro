<script lang="ts">
    import { onMount } from 'svelte';
    import { fly } from 'svelte/transition';
    import { readDir } from '@tauri-apps/plugin-fs';
    import { goto } from '$app/navigation';

    let toggle = true;
    let toggleTree = true;

    export let data: {
        path: string;
        name: string;
        note: string;
    };
    const notebookPath = data.path;
    const notebookName = data.name;

    type NoteEntry = {
        path: string;
        name?: string;
        children?: NoteEntry[];
    };
    let notes: NoteEntry[] = [];

    async function loadNotes() {
        const result = await readDir(`${notebookPath}/${notebookName}`);
        notes = (result as unknown as NoteEntry[])
        .filter(entry => entry.name)
        .map(entry => ({
            path: entry.path,
            name: entry.name
        }));
    }

    function openNote(file: string | undefined) {
      if (!file) {
        return;
      };
      
      const encodedPath = encodeURIComponent(notebookPath);
      goto(`${encodedPath}/${file}`);
    }

    onMount(async() => {
        loadNotes();
    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <div class="h-screen w-65 flex-1 absolute bg-zinc-800 transistion duration-400 ease-in-out z-0"
        class:w-0={!toggle} 
        class:w-65={toggle}>

        <div class="group flex items-center justify-center bg-zinc-800 w-9 h-9 p-1 mt-7 ml-7 rounded hover:bg-zinc-700 transition-colors duration-200 antialiased z-10"
            class:border={toggle}
            class:border-gray-500={toggle}>
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

        <div>
            {#if toggle}
            <ul class="absolute w-full font-base pt-8 antialiased z-1" class:whitespace-wrap={!toggle} transition:fly={{ x: -100 }}>
                <li class="w-full block">
                    <button class="group flex items-center w-full font-semibold hover:bg-zinc-700 px-4 py-1 transition-colors duration-200 antialiased"
                    on:click={() => toggleTree = !toggleTree}>
                        <svg 
                            class="w-5 h-5 group-hover:text-orange-600 transform transition-transform duration-200 ease-in-out"
                            class:rotate-90={toggleTree}
                            fill="none" 
                            stroke="currentColor" 
                            viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                        </svg>
                        <span class="ml-3">{notebookName}</span>
                    </button>

                    {#if toggleTree}
                    <ul class="pt-1">
                        {#each notes as note (note.name)}
                            <button class="w-full p-0.5 pl-10 flex items-center space-x-2 hover:bg-zinc-700 transistion-colors duration-200 antialiased"
                            on:click={() => openNote(note.name)}>
                                <svg
                                class="w-4 h-4 text-gray-500"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="1"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                >
                                <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                                <polyline points="14 2 14 8 20 8"/>
                                </svg>
                                <span class="">{note.name}</span>
                            </button>
                        {/each}
                    </ul>
                    {/if}
                </li>
            </ul>
            {/if}
        </div>
        
        <div class="absolute bottom-0 w-full border-t border-zinc-700 pb-10 text-xs text-zinc-400"></div>

    </div>

    <div class="mt-10 ml-70">
        <slot/>
    </div>
</main>