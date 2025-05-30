<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { fade, slide } from "svelte/transition";
    import { goto } from "$app/navigation";
    import toast, { Toaster } from 'svelte-5-french-toast';
  
    let appVersion = 'loading...';
    let email = "";
    let password = "";
    let passwordVisbility = true;

    // On initial page load
    onMount(async() => {
        appVersion = await invoke ('get_app_version');
    });

    // Redirect back to index page
    function goBack() {
        goto("../");
    }

    // Toggles password visibility
    function toggleVisibility() {
        passwordVisbility = !passwordVisbility
    }

    // Authenticates user login 
    async function verify_login() {
        // Query user credentials
        if (email.length === 0 || password.length === 0) {
            toast.error("Please fill in all fields")
            return;
        }

        try {
            const auth = await invoke<string>('verify_user', {
                email,
                password
            });

            // store new session token upon successful login
            localStorage.setItem('session_token', auth);
            goto("../../synced-vault");

        } catch (error) {
            toast.error("Incorrect credentials");
        }
    }

    // Redirects to the register page
    function register() {
        goto("./register");
    }
  </script>
  
  <main class="h-screen w-screen bg-zinc-900 text-white flex flex-col items-center justify-center space-y-6"> 
    <Toaster/>

     <!-- Title -->   
    <div out:fade={{ duration: 150 }} class="text-center">
        <h1 class="Satoshi font-bold text-5xl -mt-7">Neuro</h1>
        <p class="Satoshi text-base text-gray-400">Version: {appVersion}</p>
    </div>


    <div in:slide out:fade={{ duration: 150 }} class="w-full max-w-lg p-6">
         <!-- Back button to redirect back to index page -->
        <button class="text-gray-400 hover:text-white text-lg flex items-center space-x-2 mb-4" on:click={goBack}>
            ← <span class="pl-2"> Back</span>
        </button>

        <h2 class="text-xl font-bold mb-4">Login to Neuro Sync</h2>

         <!-- Email input -->
        <input type="text" bind:value={email} placeholder="Enter your email"
        class="w-full bg-zinc-700 text-white p-3 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-600 mb-4"/>

         <!-- Password input -->
        <div class="relative w-full mb-4">
            <input type={passwordVisbility ? "password" : "text"} bind:value={password} placeholder="Enter your password"
            class="w-full bg-zinc-700 text-white p-3 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-600"/>

            <button class="text-gray-400 absolute right-3 top-1/2 transform -translate-y-1/2 hover:text-orange-500 transistion-colors duration-200" aria-label="preview-button"
            on:click={toggleVisibility}>
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

         <!-- Submit login form button -->
        <button class="w-full bg-orange-700 text-white font-semibold py-3 rounded-md hover:bg-orange-600 transition"
        on:click={verify_login}>
            Login
        </button>

        <!-- Redirect link to regiister for an account -->
        <div class="flex justify-center text-center pt-6 ">
            <p class="text-sm text-gray-400 mb-4">Dont have an account? 
                <button class="cursor-pointer hover:underline hover:text-orange-600 transistion-colors duration-200" on:click={register}>Sign up</button>
            </p>
        </div>
    </div>
</main>
