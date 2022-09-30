<script lang="ts">
    import type { Workout } from "../../models/workout";


    export let workout: Workout = null;
</script>

{#if workout}
    <button class="workout-button" on:click>
        <div class="workout-info">
            <p class="name">{workout.name}</p>
            <p class="category">{workout.category}</p>
        </div>
    </button>
{/if}

<style>
    .workout-button {
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
</style>