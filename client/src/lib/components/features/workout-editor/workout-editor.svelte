<script lang="ts">
    import { push } from "svelte-spa-router";
    import type { Workout } from "../../../models/workout";
    import { UserStore } from "../../../stores/user-store";

    export let workout: Workout;

    async function saveWorkout() {
        const workout_row: Workout = {
            id: workout.id,
            user_id: $UserStore.id,
            name: workout.name,
            category: workout.category,
            note: workout.note,
        }
    }
</script>

{#if workout}
    <div class="page">
        <div class="content">
            <input placeholder="Routine Name" bind:value={workout.name} />
            <textarea placeholder="Notes" bind:value={workout.note} />
        </div>
    </div>
{/if}

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }

    .content {
        display: grid;
        grid: 1fr / 1fr;
        grid-gap: 1em;
        margin: 1em auto auto auto;
        max-width: 37em;
        place-items: center;
        place-content: center;
    }

    input {
        padding: 0.25em;
        width: fit-content;
        width: 100%;
        margin: 0;
        border: 1px solid black;
    }

    textarea {
        box-sizing: border-box;
        border: 1px solid black;
        padding: 0.25em;
        resize: none;
        margin: 0;
        height: 5em;
        width: 100%;
    }
</style>