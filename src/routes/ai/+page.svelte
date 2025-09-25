<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Board from "../board.svelte";
    import { type Tile } from "../tiles";
    import {createEmptyBoard} from "../tiles";

    let userBoard = createEmptyBoard();
    let enemyBoard = createEmptyBoard();

    async function onCellClick(row: Number, col: Number) {
        await invoke("update_board", {
            row,
            col,
        }).then((board) => (enemyBoard = board as Tile[][]));

        await invoke("switch_turn");

        await invoke("bot_attack").then(
            (board) => (userBoard = board as Tile[][]),
        );

        await invoke("switch_turn");
    }
</script>

<Board {userBoard} {enemyBoard} on:cellClick={(e) => onCellClick(e.detail.row, e.detail.col)} />
