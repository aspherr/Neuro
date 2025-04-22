<script lang="ts">
    import { onMount } from 'svelte';
    import { fly } from 'svelte/transition';
    import { readDir } from '@tauri-apps/plugin-fs';
    import { marked } from 'marked';
    import { invoke } from '@tauri-apps/api/core';
    import { goto } from '$app/navigation';
    import { ask } from '@tauri-apps/plugin-dialog';    
    import toast, {Toaster} from 'svelte-5-french-toast'
    import Button from '../../../../components/button.svelte';

    let toggle = true;
    let toggleTree = true;
    let toggleNote = true;
    let toggleMarkdown = false;
    let toggleCreateModal = false;
    let toggleAIModal = false;
    let userPrompt: string = '';
    let currentNote: string = '';
    let currentRemoteNote: string = '';
    let newFileName: string = '';
    let markdown: string = '';
    
    // counts words in markdown string
    $: wordCount = markdown
                    .trim()
                    .split(/\s+/)
                    .filter(word => word && !word.startsWith('#'))
                    .filter(Boolean).length;
    
    $: content = marked(markdown); // converts markdown to html
    $: session_token = localStorage.getItem("session_token");

    // User structure
    interface User {
      forename: string;
      email: string;
    }
    let account: User;
    let user_id = "";
    let notebook_id = "";
    
    // exported data for pathing
    export let data: {
        path: string;
        name: string;
        note: string;
    };
    const notebookPath = data.path;
    const notebookName = data.name;

    // Local note structure 
    type NoteEntry = {
        path: string;
        name?: string;
        children?: NoteEntry[];
    };
    let localNotes: NoteEntry[] = [];

    // Remote note structure
    type RemoteNoteEntry = {
        name: string;
    };

    let remoteNotes: RemoteNoteEntry[] = [];

    $: notes = session_token ? remoteNotes : localNotes;

    // Loads either local or remote notes
    async function loadNotes() {
        // remote notes
        if (session_token && session_token !== "null" && session_token !== "undefined") {
            remoteNotes = (await invoke<string[]>('get_note_names', { id: notebook_id }))
            .map(name => ({ name }));
            return;
        }

        // local notes
        const result = await readDir(`${notebookPath}/${notebookName}`);
        notes = (result as unknown as NoteEntry[])
        .filter(entry => entry.name)
        .map(entry => ({
            path: entry.path,
            name: entry.name
        }));
    }

    // Opens either a local or remote note
    async function openNote(file: string | undefined) {
        // remote note
        if (session_token && session_token !== "null" && session_token !== "undefined") {
            if (file) {
                currentRemoteNote = file;
            }
            let note_id = await invoke("note_id", {name: file})
            markdown = await invoke<string>('read_remote_note', { id: note_id });
            
        // local note
        } else {
            currentNote = `${notebookPath}/${notebookName}/${file}`;
            markdown = await invoke<string>('read_file', { path: currentNote });
        }

        content = marked(markdown);
        toggleNote = !toggleNote;
    }

    // Saves either a local or remote note
    async function saveNote(content: string | undefined) {
        // remote note
        if (session_token && session_token !== "null" && session_token !== "undefined") {
            let note_id = await invoke("note_id", {name: currentRemoteNote})
            await invoke('save_remote_note', {
                id:  note_id,
                content: content
            })

        // local note
        } else {
            await invoke<string>('save_file', { path: currentNote, data: content});
        }
        toast.success('File Saved!') ;
    }

    // Deletes either a local or remote note
    async function deleteNote() {
        // Confirm with user before deleting
        const confirmDelete = await ask('This action cannot be reverted. Are you sure?', {
        title: 'Delete Note',
        kind: 'warning',
        });

        if (confirmDelete) {
            // remote deletion
            if (session_token && session_token !== "null" && session_token !== "undefined") {
                let note_id = await invoke("note_id", {name: currentRemoteNote})
                let notebook_id = await invoke("notebook_id", {name: notebookName })
                await invoke('delete_remote_note', {
                    id:  note_id,
                    name: currentRemoteNote,
                    nid: notebook_id
                })
            
            // local deletion
            } else {
                await invoke<string>('delete_file', { path: currentNote }); 
            }
            toast.success('File Deleted!')
        }
        loadNotes();
    }

    // Creates either a local or remote note
    async function createNote(file: string | undefined) {
        // Remote note
        if (session_token && session_token !== "null" && session_token !== "undefined") {
            await invoke('add_note', { 
                name: `${newFileName}.md`,
                nid: notebook_id
             });
            
        // Local note
        } else {
            let filePath = `${notebookPath}/${notebookName}/${file}.md`;
            await invoke<string>('create_file', { path: filePath });
        }

        
        toggleCreateModal = false;
        newFileName = '';

        loadNotes();
        
        toggleMarkdown = true
        toast.success('File Created!')
    }

    // Toggles between edit markdown mode and preview markdown mode
    function modeNotification() {
        toggleMarkdown = !toggleMarkdown;

        if (toggleMarkdown) {
            toast.success('Editor Mode', {
                icon: '🖊️'
            })
        
        } else {
            toast.success('Preview Mode', {
                icon: '👁️'
            }) 
        }
    }

    // OpenAI API call for Neuro prompt
    async function ask_neuro(prompt: string) {
        // adjustment to user's base prompt
        let adjustedPrompt = `${prompt}\n\nPlease respond in markdown and dont exceed 3 paragraphs.`;
        const response = await toast.promise(
            invoke<string>("neuro", { prompt: adjustedPrompt }),
            {
                loading: 'Neuro is thinking...',
                success: 'Your answer has arrived!',
                error: 'Sorry there was a problem!',
            }
        );

        markdown += `\n\n${response}`
        toggleAIModal = !toggleAIModal;
        userPrompt = "";
    }

    // Redirects back to notebook page
    function goBack() {
        goto('./');
    }

    // On initial page load
    onMount(async() => {
        // setting user session data
        if (session_token && session_token !== "null" && session_token !== "undefined") {
            account = await invoke('get_user_data', { sessionToken: session_token });
            user_id = await invoke('get_id', {email: account.email});
            notebook_id = await invoke('notebook_id', {name: notebookName})
        }

        loadNotes();

        // opens top file upon load
        let firsFilePath = await invoke<string>('get_first_file', {
            path: `${notebookPath}/${notebookName}`}
        )

        if (firsFilePath) {
            openNote(firsFilePath);
        }
    });

</script>

<main class="h-screen w-screen bg-zinc-900 text-white flex flex-col">
    <div class="h-screen w-65 flex-1 absolute bg-zinc-800 transistion duration-400 ease-in-out z-0"
        class:w-0={!toggle} 
        class:w-65={toggle}> <!-- toggle sidebar -->
        
        {#if !toggleAIModal}
            <Toaster />
        {/if}

        <div class="flex items-start mx-7 my-7 transition-all duration-400 ease-in-out" 
        class:flex-row={toggle}
        class:flex-col={!toggle}
        class:space-x-11={toggle}
        class:space-y-5={!toggle}> <!-- top row buttons -->

            <!-- hamburger to toggle side bar -->
            <Button clickEvent={() => toggle = !toggle}>
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
            </Button>

            <!-- create file button -->
            <Button clickEvent={() => toggleCreateModal = !toggleCreateModal}>
                <svg xmlns="http://www.w3.org/2000/svg"
                class="group-hover:text-orange-500 transistion-colors duration-200"
                width="24" height="24"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
                </svg>
            </Button>

            <!-- Delete file button -->
            <Button clickEvent={deleteNote}>
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
            </Button>
        </div>

        <div>
            <div class="absolute w-full border-t border-zinc-700 text-xs text-zinc-400 z-10"></div>

            <!-- file tree -->
            {#if toggle}
            <ul class="absolute w-full font-base antialiased z-1" class:whitespace-wrap={!toggle} transition:fly={{ x: -100 }}>
                <li class="w-full block mt-5">
                    <!-- arrow that can hide/show file tree -->
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
                        
                        <!-- Notebook name -->
                        <span class="ml-3">{notebookName}</span>
                    </button>

                    {#if toggleTree}
                    <ul class="pt-1">
                        <!-- Loops through all files and lists them as a tree -->
                        {#each notes as note (note.name)}
                            <button class="w-full p-0.5 pl-10 flex items-center space-x-2 hover:bg-zinc-700 transistion-colors duration-200 antialiased"
                            on:click={() => openNote(note.name)}> <!-- Open up text editor for that note -->
                                <svg
                                class="w-4 h-4 text-gray-500"
                                class:text-orange-600={note.name == currentNote.split("/").pop()}
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
            class:my-[760px]={toggle}
            class:my-[630px]={!toggle}
            class:pt-23={toggle}
            class:pt-0={!toggle}
            class:flex-row={toggle}
            class:flex-col-reverse={!toggle}
            class:space-x-11={toggle}
            class:space-y-5={!toggle}
            class:space-y-reverse={!toggle}> <!-- Bottom row buttons -->
            
            <!-- Back button that redirects back to notebooks page -->
            <Button clickEvent={goBack}>
                <svg 
                    class="w-5 h-5 group-hover:text-orange-600 transform transition-transform duration-200 ease-in-out"
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 5l-7 7 7 7" />
                </svg>
            </Button>

            <!-- Save button to write changes -->
            <Button clickEvent={() => saveNote(markdown)}>
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
            </Button>

            <!-- 'Neuro' button to ask for AI assistance -->
            <Button clickEvent={() => toggleAIModal = !toggleAIModal}>
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
            </Button>
        </div>
        
    </div>

    <!-- Preview button that toggles between edit and preview mode -->
    <Button clickEvent={modeNotification} extraClasses=" ml-auto mt-7 mr-7">
        <svg 
        xmlns="http://www.w3.org/2000/svg"
        class="group-hover:text-orange-500 transistion-colors duration-200"
        viewBox="0 0 24 24" 
        width="24" height="24" 
        fill="none" 
        stroke="currentColor" 
        stroke-width="2" 
        stroke-linecap="round" 
        stroke-linejoin="round">
            <path d="M1 12s4-7 11-7 11 7 11 7-4 7-11 7S1 12 1 12z" />
            <circle cx="12" cy="12" r="3" />
        </svg>  
    </Button>

    <!-- Text editor -->
    <div class="prose prose-invert transition-all duration-400 ease-in-out" 
    class:ml-30={!toggle}
    class:ml-70={toggle}
    class:display-none={!toggleNote}>
        {#if toggleMarkdown}
            <textarea class="w-[70vw] h-[100vh] p-4 focus:outline-none focus:none resize-none font-mono" 
            bind:value={markdown} placeholder="Type Markdown here..."></textarea> <!-- Default text if nothing in written -->

        {:else}
            <!-- load markdown as html -->
            {@html content}
        {/if}

        <!-- word counter -->
        <div class="absolute bottom-0 right-4 pr-3">
            <p class="flex items-center justify-center bg-zinc-900 text-center text-sm w-25 h-8 rounded-4xl border border-gray-500">
                {#if wordCount == 1}
                    {wordCount} Word
                
                {:else}
                    {wordCount} Words
                {/if}
            </p>
        </div>
    </div>

    <!-- Create file modal -->
    {#if toggleCreateModal}
    <div class="fixed inset-0 backdrop-blur-md flex items-center justify-center z-50">
      <div class="bg-zinc-800 text-white p-6 rounded-lg w-full max-w-md shadow-lg">
        <h2 class="text-xl font-bold mb-4">Create a New Note</h2>

        <!-- Input field -->
        <div class="relative">
            <input
            type="text"
            bind:value={newFileName}
            placeholder="Note name"
            class="w-full p-3 rounded-md bg-zinc-700 text-white focus:outline-none focus:ring-2 focus:ring-orange-600"
            />
            <span class="absolute right-0 top-1/2 -translate-y-1/2 text-gray-400 pointer-events-none bg-zinc-900 p-3 rounded-r">
                .md
            </span>
        </div>

        <!-- Cancel button -->
        <div class="flex justify-end gap-2 mt-6">
          <button class="px-4 py-2 bg-zinc-600 rounded-md hover:bg-zinc-700 transition"
            on:click={() => toggleCreateModal = false}>
            Cancel
          </button>

          <!-- Creates note -->
          <button
            class="px-4 py-2 bg-orange-700 rounded-md hover:bg-orange-600 transition"
            on:click={() => {createNote(newFileName)}}>
            Create
          </button>
        </div>
      </div>
    </div>
    {/if}

    <!-- Neuro prompt modal -->
    {#if toggleAIModal}
        <Toaster />
        <div class="fixed inset-0 backdrop-blur-md flex items-center justify-center z-50">
        <div class="bg-zinc-800 text-white p-6 rounded-lg w-full max-w-md shadow-lg">
            <h2 class="text-xl font-bold mb-4">Hi, Neuro here! 😊</h2>

            <!-- Input field -->
            <div class="relative">
                <input
                type="text"
                bind:value={userPrompt}
                placeholder="Ask away..."
                class="w-full p-3 rounded-md bg-zinc-700 text-white focus:outline-none focus:ring-2 focus:ring-orange-600"
                />
            </div>

            <!-- Cancel button -->
            <div class="flex justify-end gap-2 mt-6">
            <button class="px-4 py-2 bg-zinc-600 rounded-md hover:bg-zinc-700 transition"
                on:click={() => toggleAIModal = false}>
                Cancel
            </button>

            <!-- Injects AI response into current file -->
            <button
                class="px-4 py-2 bg-orange-700 rounded-md hover:bg-orange-600 transition"
                on:click={() => {ask_neuro(userPrompt)}}>
                Think
            </button>
            </div>
        </div>
        </div>
    {/if}
</main> 