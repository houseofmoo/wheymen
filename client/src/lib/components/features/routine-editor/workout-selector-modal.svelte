<script lang="ts">
    import { createEventDispatcher} from "svelte";
    import type { Workout } from "../../../models/workout";
    import Modal from "../../display/modal.svelte";

    export let workouts: Workout[]= [];
    const dispatch = createEventDispatcher();
    let showModal: () => void;
    let closeModal: () => void;

    function workoutSelected(workout: Workout) {
        dispatch('workout-selected', workout);
        closeModal();
    }
</script>

<button class="control-button" on:click={() => showModal()}>add workout</button>
<Modal bind:showModal={showModal} bind:closeModal={closeModal}>
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