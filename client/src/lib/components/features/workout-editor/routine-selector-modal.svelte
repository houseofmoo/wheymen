<script lang="ts">
    import { createEventDispatcher} from "svelte";
    import type { Routine } from "../../../models/routine";

    export let routines: Routine[]= [];
    const dispatch = createEventDispatcher();
    let modal;

    function workoutSelected(routine: Routine) {
        dispatch('routine-selected', routine);
        modal.close();
    }
</script>

<button class="control-button" on:click={() => modal.showModal()}>add to routine</button>

<dialog class="modal" bind:this={modal}>
    <button class="close-button" on:click={() => modal.close()}>x</button>
    <div class="routine-selector">
        {#if routines.length <= 0}
            <p class="unavailable">No routines available to select</p>
        {:else}
            {#each routines as routine}
                <button class="routine-button" on:click={() => workoutSelected(routine)}>
                    <p class="center-text large-text">{routine.name}</p>
                </button>
            {/each}
        {/if}
    </div>
</dialog>

<style>
    .modal {
        background-color: var(--primary-color-800);
        border: 1px solid black;
        width: 60vw;
        max-width: 60em;
    }

    .modal::backdrop {
        background: black;
        opacity: 0.4;
    }

    .routine-selector {
        display: grid;
        grid: 1fr / 1fr;
        place-items: center;
        place-content: center;
        width: 100%;
        height: fit-content;
        overflow-y: scroll;
        scrollbar-width: none;
        z-index: 2;
    }

    .close-button {
        display: flex;
        margin-left: auto;
        margin-top: 0;
        padding-top: 0;
        border: none;
        color: var(--secondary-color);
        background-color: var(--primary-color-800);
        text-align: right;
    }

    .routine-button, .control-button {
        width: 100%;
    }

    .unavailable {
        text-align: center;
        color: var(--text-color);
    }
</style>