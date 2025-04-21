<script lang="ts">
    export let clickEvent = () => {};
    export let extraClasses = "";
</script>

<div class="group flex items-center justify-center bg-zinc-800 border border-gray-500 
    w-9 h-9 p-1 rounded hover:bg-zinc-700 transition-all duration-400 ease-in-out transform antialiased z-10
    ${extraClasses}">
    <button class="text-gray-400" aria-label="create-button" on:click={clickEvent}>
        <slot/>
    </button>
</div>
