<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { Workout } from "../../../models/workout";

    export let workouts: Workout[]= [];
    const dispatch = createEventDispatcher();
    let visible = false;

    function workoutSelected(workout: Workout) {
        visible = false;
        dispatch('workout-selected', workout);
    }
</script>

{#if workouts.length > 0}
    <button class="add-workout" on:click={() => visible = !visible }>add workout</button>
{/if}

{#if visible}
    <div class="window">
        <div class="workout-selector">
            {#each workouts as workout}
                <button class="workout-button" on:click={() => workoutSelected(workout)}>
                    <p class="large-text">{workout.name}</p>
                    <p class="small-text">{workout.category}</p>
                </button>
            {/each}
        </div>
    </div>
{/if}

<style>
    .add-workout {
        width: 100%;
    }

    .window {
        position: absolute;
        top: 0%;
        width: 100%;
        max-width: 58em;
        min-width: 360px;
        height: 100vh;
        background-color: var(--lightgrey);
        z-index: 1;
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

    p {
        text-align: center;
    }

    .workout-button {
        width: 100%;
        margin: 0;
        padding: 0;
    }
</style>