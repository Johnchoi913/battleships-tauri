<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Board from "../../board.svelte";
    import { onMount } from "svelte";
    import { type Tile } from "../../tiles";
    import { createEmptyBoard } from "../../tiles";

    let userBoard = createEmptyBoard();
    let enemyBoard = createEmptyBoard();

    async function onCellClick(row: number, col: number) {
        await invoke("update_board", {
            row,
            col,
        }).then((board) => (userBoard = board as Tile[][]));
        await invoke("switch_turn");
    }

    onMount(async () => {
        invoke("start_server");
    });
</script>

<Board
    {userBoard}
    {enemyBoard}
    on:cellClick={(e) => onCellClick(e.detail.row, e.detail.col)}
/>
