<script lang="ts">
    import { onMount } from 'svelte';
    import { fly } from 'svelte/transition';
    import { readDir } from '@tauri-apps/plugin-fs';
    import { marked } from 'marked';
    import { invoke } from '@tauri-apps/api/core';
    import { goto } from '$app/navigation';

    let toggle = true;
    let toggleTree = true;
    let toggleNote = true;
    let markdown = '';
    let content: string | Promise<string> | null = null;

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

    async function openNote(file: string | undefined) {
        let filePath = `${notebookPath}/${notebookName}/${file}`;
        const out = await invoke<string>('read_file', { path: filePath });
        markdown = out;
        content = marked(markdown);
        toggleNote = !toggleNote;
    }

    function goBack() {
        goto('./');
    }

    onMount(async() => {
        loadNotes();
    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <div class="h-screen w-65 flex-1 absolute bg-zinc-800 transistion duration-400 ease-in-out z-0"
        class:w-0={!toggle} 
        class:w-65={toggle}>

        <div class="flex items-start mx-7 my-7 transition-all duration-400 ease-in-out" 
        class:flex-row={toggle}
        class:flex-col={!toggle}
        class:space-x-11={toggle}
        class:space-y-5={!toggle}>

            <div class="group flex items-center justify-center bg-zinc-800 w-9 h-9 p-1 rounded hover:bg-zinc-700 transition-all duration-400 ease-in-out transform antialiased z-10"
            class:border={toggle}
            class:border-gray-500={toggle}>
            <button class="text-gray-400" aria-label="toggle-button" on:click={() => toggle = !toggle}>
                <svg
                  class="w-7 h-7 group-hover:text-orange-500 transistion-colors duration-200"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 24 24">
                    <path d="M4 6h16M4 12h16M4 18h16"/>
                </svg>
              </button>
            </div>

            <div class="group flex items-center justify-center bg-zinc-800 w-9 h-9 p-1 rounded hover:bg-zinc-700 transition-all duration-400 ease-in-out transform antialiased z-10"
            class:border={toggle}
            class:border-gray-500={toggle}>
                <button class="text-gray-400" aria-label="create-button">
                    <svg xmlns="http://www.w3.org/2000/svg"
                    class="group-hover:text-orange-500 transistion-colors duration-200"
                    width="24" height="24"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    stroke-width="1.5">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
                    </svg>
                </button>
            </div>

            <div class="group flex items-center justify-center bg-zinc-800 w-9 h-9 p-1 rounded hover:bg-zinc-700 transition-all duration-400 ease-in-out transform antialiased z-10"
            class:border={toggle}
            class:border-gray-500={toggle}>
                <button class="text-gray-400" aria-label="delete-button">
                    <svg xmlns="http://www.w3.org/2000/svg"
                    class="group-hover:text-orange-500 transistion-colors duration-200"
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

        </div>

        <div>
            {#if toggle}
            <ul class="absolute w-full font-base antialiased z-1" class:whitespace-wrap={!toggle} transition:fly={{ x: -100 }}>
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
        
        <div class="absolute bottom-0 w-full border-t border-zinc-700 pb-18 text-xs text-zinc-400"></div>

        <div class="flex items-start mx-7 transition-all duration-400 ease-in-out transform"
            class:my-[855px]={toggle}
            class:my-[625px]={!toggle}
            class:flex-row={toggle}
            class:flex-col-reverse={!toggle}
            class:space-x-11={toggle}
            class:space-y-5={!toggle}
            class:space-y-reverse={!toggle}>
            
            <div class="group flex items-center justify-center bg-zinc-800 w-9 h-9 p-1 rounded hover:bg-zinc-700 transition-all duration-400 ease-in-out transform antialiased z-10"
            class:border={toggle}
            class:border-gray-500={toggle}>
                <button class="text-gray-400" aria-label="toggle-button" on:click={goBack}>
                    <svg 
                    class="w-5 h-5 group-hover:text-orange-600 transform transition-transform duration-200 ease-in-out"
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 5l-7 7 7 7" />
                    </svg>
                </button>
            </div>

            <div class="group flex items-center justify-center bg-zinc-800 w-9 h-9 p-1 rounded hover:bg-zinc-700 transition-all duration-400 ease-in-out transform antialiased z-10"
            class:border={toggle}
            class:border-gray-500={toggle}>
                <button class="text-gray-400" aria-label="save-button">
                    <svg xmlns="http://www.w3.org/2000/svg"
                    class="group-hover:text-orange-600 transform transition-transform duration-200 ease-in-out"
                    viewBox="0 0 24 24" 
                    width="20" height="20" 
                    fill="none" 
                    stroke="currentColor" 
                    stroke-width="1.5" 
                    stroke-linecap="round" 
                    stroke-linejoin="round">
                        <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" />
                        <polyline points="17 21 17 13 7 13 7 21" />
                        <polyline points="7 3 7 8 15 8" />
                      </svg>                      
                </button>
            </div>

            <div class="group flex items-center justify-center bg-zinc-800 w-9 h-9 p-1 rounded hover:bg-zinc-700 transition-all duration-400 ease-in-out transform antialiased z-10"
            class:border={toggle}
            class:border-gray-500={toggle}>
                <button class="text-gray-400" aria-label="toggle-button">
                    <svg xmlns="http://www.w3.org/2000/svg"
                    class="group-hover:text-orange-600 transform transition-transform duration-200 ease-in-out"
                    viewBox="0 0 24 24" 
                    width="22" height="22" 
                    fill="none" 
                    stroke="currentColor" 
                    stroke-width="1.5" 
                    stroke-linecap="round" 
                    stroke-linejoin="round">
                        <rect x="4" y="8" width="16" height="12" rx="2" ry="2" />
                        <circle cx="9" cy="14" r="1" />
                        <circle cx="15" cy="14" r="1" />
                        <line x1="12" y1="4" x2="12" y2="8" />
                        <circle cx="12" cy="4" r="1" />
                      </svg>
                </button>
            </div>
        </div>
        
    </div>

    <div class="prose prose-invert mt-10 ransition-all duration-400 ease-in-out" 
    class:ml-30={!toggle}
    class:ml-70={toggle}
    class:display-none={!toggleNote}>
        {@html content}
    </div>
    
</main>  

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <!-- Robot head -->
    <rect x="4" y="8" width="16" height="12" rx="2" ry="2" />
    
    <!-- Eyes -->
    <circle cx="9" cy="14" r="1" />
    <circle cx="15" cy="14" r="1" />
    
    <!-- Antenna -->
    <line x1="12" y1="4" x2="12" y2="8" />
    <circle cx="12" cy="4" r="1" />
  </svg>
  