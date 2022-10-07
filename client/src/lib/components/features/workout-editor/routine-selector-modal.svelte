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
    <div class="selector">
        {#if routines.length <= 0}
            <p class="unavailable">No routines available to select</p>
        {:else}
            {#each routines as routine}
                <button class="control-button" on:click={() => workoutSelected(routine)}>
                    <p class="center-text">{routine.name}</p>
                    <p class="left-text">{routine.name}</p>
                </button>
            {/each}
        {/if}
    </div>
</dialog>

<style>
    .modal {
        background-color: var(--primary-color-800);
        border: 1px solid black;
        max-width: 30em;
        width: 100%;
    }

    .modal::backdrop {
        background: black;
        opacity: 0.4;
    }

    .close-button {
        display: flex;
        margin-left: auto;
        margin-top: 0;
        padding-top: 0;
        border: none;
        color: var(--secondary-color);
        background-color: var(--primary-color-800);
    }

    .selector {
        scrollbar-width: none;
    }

    .control-button {
        width: 100%;
        margin: 0.25em 0;
    }

    .unavailable {
        text-align: center;
        color: var(--text-color);
    }
</style>