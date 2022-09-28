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
    <div class="workout-selector">
        {#each workouts as workout}
            <button class="workout-button" on:click={() => workoutSelected(workout)}>
                <p class="center-text large-text">{workout.name}</p>
                <p class="center-text small-text">{workout.category}</p>
            </button>
        {/each}
    </div>
</dialog>

<style>
    .modal {
        background-color: var(--darkgrey);
        border: 1px solid black;
        width: 60vw;
        max-width: 60em;
    }

    .modal::backdrop {
        background: black;
        opacity: 0.4;
    }

    .workout-selector {
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
        color: var(--orange);
        background-color: var(--darkgrey);
        text-align: right;
    }

    .workout-button, .control-button {
        width: 100%;
    }
</style>