<script lang="ts">
    import { createEventDispatcher} from "svelte";
    import type { Routine } from "../../../models/routine";
    import Modal from "../../display/modal.svelte";

    export let routines: Routine[]= [];
    const dispatch = createEventDispatcher();
    let showModal: () => void;
    let closeModal: () => void;

    function workoutSelected(routine: Routine) {
        dispatch('routine-selected', routine);
        closeModal();
    }
</script>

<button class="control-button" on:click={() => showModal()}>add to routine</button>
<Modal bind:showModal={showModal} bind:closeModal={closeModal}>
    {#if routines.length <= 0}
        <p class="unavailable">No routines available to select</p>
    {:else}
        {#each routines as routine}
            <button class="control-button" on:click={() => workoutSelected(routine)}>
                <p class="center-text">{routine.name}</p>
            </button>
        {/each}
    {/if}
</Modal>

<style>
    .control-button {
        width: 100%;
        margin: 0.25em 0;
    }

    .unavailable {
        text-align: center;
        color: var(--text-color);
    }
</style>