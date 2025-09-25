<script>
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { invoke } from "@tauri-apps/api/core";
    import "../global.css";

    let { children } = $props();
</script>

{#if $page.url.pathname !== "/"}
    <button class="back-button" on:click={() => goto("/")}>Back</button>
{:else}
    <button class="back-button" on:click={() => invoke("quit_app")}>Quit</button
    >
{/if}

{@render children()}

<style>
    .back-button {
        position: fixed;
        top: 1rem;
        left: 1rem;
        z-index: 1000;
        width: fit-content;
        height: fit-content;
    }
</style>
