<script>
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
</script>

<div class="container">
    <button
        on:click={() => {
            invoke("reset_board");
            goto("/ai");
        }}
    >
        Play against AI
    </button>
    <button
        on:click={() => {
            invoke("reset_board");
            goto("/online");
        }}>Online Multiplayer</button
    >
</div>

<style>
    .container {
        background-color: black;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 1.5rem;
        height: 100vh;
        width: 50vw;
    }
    button {
        width: 100%;
        padding: 1rem;
    }
</style>
