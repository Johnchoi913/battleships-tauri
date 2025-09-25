<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { type Tile } from "./tiles";
    import { createEventDispatcher } from 'svelte';
    
    
    const dispatch = createEventDispatcher();

    let loading = false;
    async function handleTileClick(row: number, col: number) {
        if (loading) {
            return;
        }

        dispatch('cellClick', { row, col });  

        loading = true;
        if (enemyBoard[row][col].type !== "Empty" && enemyBoard[row][col].type !== "Ship") {
            loading = false;
            return;
        }
        loading = false;
    }

    onMount(() => {
        invoke("get_boards").then((boards) => {
            userBoard = (boards as { userBoard: Tile[][]; enemyBoard: Tile[][] }).userBoard;
            enemyBoard = (boards as { userBoard: Tile[][]; enemyBoard: Tile[][] }).enemyBoard;
        });
    });

    export let userBoard = Array.from({ length: 10 }, () =>
        Array.from({ length: 10 }, () => ({ type: "Empty" })),
    );

    export let enemyBoard = Array.from({ length: 10 }, () =>
        Array.from({ length: 10 }, () => ({ type: "Empty" })),
    );

</script>

<div class="gameContainer">
    <div class="board">
        {#each enemyBoard as row, rowIndex}
            {#each row as tile, colIndex}
                {#if tile.type === "Empty"}
                    <div
                        class="tile empty"
                        on:click={() => handleTileClick(rowIndex, colIndex)}
                    ></div>
                {:else if tile.type === "Ship"}
                    <div
                        class="tile empty"
                        on:click={() => handleTileClick(rowIndex, colIndex)}
                    ></div>
                {:else if tile.type === "Dead"}
                    <div class="tile dead"></div>
                {:else if tile.type === "Hit"}
                    <div class="tile hit"></div>
                {:else if tile.type === "Miss"}
                    <div class="tile miss"></div>
                {/if}
            {/each}
        {/each}
    </div>
    <div class="board">
        {#each userBoard as row, rowIndex}
            {#each row as tile, colIndex}
                {#if tile.type === "Empty"}
                    <div class="tile empty"></div>
                {:else if tile.type === "Ship"}
                    <div class="tile ship"></div>
                {:else if tile.type === "Dead"}
                    <div class="tile dead"></div>
                {:else if tile.type === "Hit"}
                    <div class="tile hit"></div>
                {:else if tile.type === "Miss"}
                    <div class="tile miss"></div>
                {/if}
            {/each}
        {/each}
    </div>
</div>

<style>
    .gameContainer {
        display: grid;
        flex-wrap: wrap;
        align-items: center;
        justify-items: center;
        grid-gap: 10vh;
    }
    .board {
        aspect-ratio: 1 / 1;
        background-color: lightsteelblue;
        border: 2px solid lightsteelgrey;
        display: grid;
        grid-template-columns: repeat(10, 1fr);
        grid-template-rows: repeat(10, 1fr);
        width: min(40vh, 40vw);
        gap: 4px;
        box-sizing: border-box;
    }
    .tile {
        width: 100%;
        height: 100%;
        color: white;
        border: 2px navajowhite solid;
        justify-items: center;
        align-items: center;
        box-sizing: border-box;
        text-align: center;
        text-justify: center;
        transition:
            transform 0.2s ease,
            background-color 0.3s ease,
            box-shadow 0.3s ease;
    }
    .dead {
        background-color: #ff4747; /* Red for dead tiles */
    }
    .hit {
        background-color: #6a4f31; /* Dark brown for ships */
    }
    .empty {
        background-color: #4e5b6e; /* Muted blue for empty tiles */
    }
    .miss {
        background-color: #b0b0b0; /* Light grey for misses */
    }
    .ship {
        background-color: #90EE90; /* Light green for ships on your side */
    }
    .tile:hover {
        transform: scale(1.1);
        background-color: rgba(255, 255, 255, 0.1);
        box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.3);
    }
</style>
