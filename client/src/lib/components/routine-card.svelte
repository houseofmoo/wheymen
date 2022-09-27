<script lang="ts">
    import { push } from "svelte-spa-router";
    import type { Routine } from "../models/routine";
    import { cardClicked } from "../utils/actions";

    export let routine: Routine = null;

    function editRoutine() {
        push(`/edit-routine/${routine.id}`);
        return;
    }
</script>

{#if routine}
    <div class="card">
        <p class="routine-name large-text">{routine.name}</p>
        <div class="routine-info">
            <div>
                <p class="small-text">last completed:</p>
                <p class="small-text">{new Date(routine.last_completed).toLocaleDateString()}</p>
                <br />
                <p class="small-text">{routine.days}</p>
            </div>
            <div>
                {#each routine.workouts as workout}
                    <p class="small-text">{workout.name}</p>
                {/each}
            </div>
        </div>
        <button on:click={editRoutine}>Edit</button>
    </div>
{/if}

<style>
    .card {
        background-color: var(--lightgrey);
        border: 1px solid black;
        padding: 0.5em 1em;
        margin: 1em;
    }

    .routine-name {
        margin: 0.5em;
        text-align: center;
    }

    .routine-info {
        display: grid;
        grid: 1fr / 1fr 1fr;
    }

    p {
        margin: 0;
        padding: 0;
    }
</style>