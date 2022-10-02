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
        {#if workouts.length <= 0}
            <p class="unavailable">No workouts available to select</p>
        {:else}
            {#each workouts as workout}
                <button class="workout-button" on:click={() => workoutSelected(workout)}>
                    <div class="workout-info">
                        <p class="name">{workout.name}</p>
                        <p class="category">{workout.category}</p>
                    </div>
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
        color: var(--secondary-color);
        background-color: var(--primary-color-800);
        text-align: right;
    }

    .workout-button, .control-button {
        width: 100%;
        margin: 0.25em 0;
    }

    .workout-info {
        display: grid;
        grid: auto / 1fr 1fr;
        color: var(--text-color);
    }

    .name {
        text-align: left;
        margin-left: 1em;
    }

    .category {
        text-align: right;
        margin-right: 1em;
    }  
    
    .unavailable {
        text-align: center;
        color: var(--text-color);
    }
</style>