<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Board from "../../board.svelte";
    import { type Tile } from "../../tiles";
    import { createEmptyBoard } from "../../tiles";

    let userBoard = createEmptyBoard();
    let enemyBoard = createEmptyBoard();

    async function onCellClick(row: number, col: number) {
        await invoke("update_board", {
            row,
            col,
        }).then((board) => (enemyBoard = board as Tile[][]));

        await invoke("switch_turn");
    }

    let joined = false;
    let ipAddress = "";
</script>

{#if joined}
    <Board
        {enemyBoard}
        {userBoard}
        on:cellClick={(e) => onCellClick(e.detail.row, e.detail.col)}
    />
{:else}
    <div class="container">
        <h1>Join Game</h1>
        <p>Enter the IP address of the host to join the game.</p>
        <input
            class="ipField"
            type="text"
            placeholder="Enter IP Address"
            bind:value={ipAddress}
        />
        <button
            on:click={async () => {
                await invoke("connect_to_game", { ip: ipAddress })
                    .then(() => {
                        joined = true;
                    })
                    .catch((error) => {
                        console.error("Error joining game:", error);
                        alert(
                            "Failed to join game. Please check the IP address.",
                        );
                    });
            }}>Join Game</button
        >
    </div>
{/if}

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
    .ipField {
        margin: 20px;
        font-size: 1.2em;

        color: aliceblue;
        background-color: #333;
        width: inherit;
        padding: 10px;
        border-radius: 5px;
        border: 1px solid #555;
    }
</style>
