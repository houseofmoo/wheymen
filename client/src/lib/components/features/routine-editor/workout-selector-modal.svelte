<script lang="ts">
    import { createEventDispatcher} from "svelte";
    import type { Workout } from "../../../models/workout";

    export let workouts: Workout[]= [];
    const dispatch = createEventDispatcher();
    let modal;

    function workoutSelected(workout: Workout) {
        dispatch('workout-selected', workout);
        modal.close();
    }
</script>

<button class="control-button" on:click={() => modal.showModal()}>add workout</button>

<dialog class="modal" bind:this={modal}>
    <button class="close-button" on:click={() => modal.close()}>x</button>
    <div class="selector">
        {#if workouts.length <= 0}
            <p class="unavailable">No workouts available to select</p>
        {:else}
            {#each workouts as workout}
                <button class="control-button" on:click={() => workoutSelected(workout)}>
                    <p class="center-text">{workout.name}</p>
                    <p class="left-text">{workout.category}</p>
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