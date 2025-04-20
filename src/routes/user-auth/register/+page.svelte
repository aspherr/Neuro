<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { fade, slide } from "svelte/transition";
    import { goto } from "$app/navigation";
    import { toast, Toaster } from 'svelte-5-french-toast';
  
    let appVersion = 'loading...';
    
    let forename = "";
    let email = "";
    let password = "";
    let confirmPass = "";

    let passwordVisbility = false;
    let confirmPasswordVisbility = false;

    // taken from: https://uibakery.io/regex-library/email
    const emailRegex = /^\S+@\S+\.\S+$/;
    let validEmail = false;
        
    onMount(async() => {
        appVersion = await invoke ('get_app_version');
    });

    function validateEmail() {
        // true only if email conforms to regex condition
        validEmail = emailRegex.test(email);
    }

    function toggleVisibility(visbility: 'main-pass' | 'confirm-pass') {
        if (visbility === 'main-pass') {
            passwordVisbility = !passwordVisbility;
        
        } else if (visbility === 'confirm-pass') {
            confirmPasswordVisbility = !confirmPasswordVisbility;
        }
    }
    
    async function create_account() {
        if (forename.length == 0) {
            toast.error("Please enter your forename");
            return;
        
        } else if (!validEmail) {
            toast.error("Enter a valid email");
            return;
        
        } else if (password.length === 0 || confirmPass.length === 0) {
            toast.error("Enter a password");
            return;
        
        } else if (password != confirmPass) {
            toast.error("Passwords do not match");
            return;
        }
    }

    function login() {
        goto("./login");
    }   
  </script>
  
  <main class="h-screen w-screen bg-zinc-900 text-white flex flex-col items-center justify-center space-y-6">
    <Toaster/>

    <div out:fade={{ duration: 150 }} class="text-center">
        <h1 class="Satoshi font-bold text-5xl mt-7">Neuro</h1>
        <p class="Satoshi text-base text-gray-400">Version: {appVersion}</p>
    </div>

    <div in:slide out:fade={{ duration: 150 }} class="w-full max-w-lg p-6">
        <h2 class="text-xl font-bold mb-4">Create a Neuro Sync account</h2>

        <input type="text" bind:value={forename} placeholder="Enter your forename"
        class="w-full bg-zinc-700 text-white p-3 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-600 mb-4"/>

        <div class="relative w-full mb-4">
            <input type="text" bind:value={email} placeholder="Enter your email"
            class="w-full bg-zinc-700 text-white p-3 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-600"
            on:input={validateEmail}/>

            <span class="absolute w-2.5 h-2.5 rounded-full right-3 top-1/2 transform -translate-y-1/2 mr-2 mt-0.5 antialiased"
             class:bg-green-500={validEmail}
             class:bg-red-500={!validEmail}
             class:bg-zinc-800={email.length === 0}>
            </span>
        </div>

        <div class="relative w-full mb-4">
            <input type={passwordVisbility ? "text" : "password"} bind:value={password} placeholder="Enter your password"
            class="w-full bg-zinc-700 text-white p-3 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-600"/>

            <button class="text-gray-400 absolute right-3 top-1/2 transform -translate-y-1/2 hover:text-orange-500 transition-colors duration-200" aria-label="preview-button"
            on:click={() => {toggleVisibility('main-pass')}}>
                <svg 
                xmlns="http://www.w3.org/2000/svg"
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
            </button>
        </div>

        <div class="relative w-full mb-4">
            <input type={confirmPasswordVisbility ? "text" : "password"} bind:value={confirmPass} placeholder="Confirm your password"
            class="w-full bg-zinc-700 text-white p-3 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-600"/>

            <button class="text-gray-400 absolute right-3 top-1/2 transform -translate-y-1/2 hover:text-orange-500 transition-colors duration-200" aria-label="preview-button"
            on:click={() => {toggleVisibility('confirm-pass')}}>
                <svg 
                xmlns="http://www.w3.org/2000/svg"
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
            </button>
        </div>

        <button class="w-full bg-orange-700 text-white font-semibold py-3 mt-3 rounded-md hover:bg-orange-600 transition"
        on:click={create_account}>
            Create Account
        </button>

        <div class="flex justify-center text-center pt-6 ">
            <p class="text-sm text-gray-400 mb-4">Have have an account? 
                <button class="cursor-pointer hover:underline hover:text-orange-600 transistion-colors duration-200" on:click={login}>Login</button>
            </p>
        </div>
    </div>
</main>
