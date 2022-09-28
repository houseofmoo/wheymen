<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { getAllWorkouts } from "../api/workout";
    import { UserStore } from "../stores/user-store";
    import { getAllRoutines } from "../api/routine";
    import type { Routine } from "../models/routine";
    import type { Workout } from "../models/workout";
    import RoutineCard from "../components/routine-card.svelte";
    import Title from "../components/display/title.svelte";

    let routines: Routine[] = [];
    let workouts: Workout[] = [];

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
        }

        // get users routines
        const routine_res = await getAllRoutines($UserStore);
        if (routine_res.count > 0) {
            routines = routine_res.result;
        } else {
            // TODO: maybe no routines, maybe error occured
        }

        const workout_res = await getAllWorkouts($UserStore);
        if (workout_res.count > 0) {
            workouts = workout_res.result;
        } else {
            // TODO: maybe no workouts, maybe error occured
        }
    });

 
</script>

{#if $UserStore}
    <div class="page">
        <Title subtitle={"profile"} />
        <div class="action-buttons">
            <button on:click={() => push('/account')}>account</button>
            <button on:click={() => push('/create-routine')}>+routine</button>
            <button on:click={() => push('/create-workout')}>+workout</button>
        </div>
        {#each routines as routine}
            <RoutineCard routine={routine} />
        {/each}

        {#each workouts as workout}
            <button on:click={() => push(`/edit-workout/${workout.id}`)}>{workout.name}</button>
        {/each}
    </div>
{/if}

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }

    .action-buttons {
        display: grid;
        grid: 1fr / repeat(3, 1fr);
        place-content: center;
        place-items: center;
        grid-gap: 1em;
    }

    .action-buttons > button {
        width: 100%;
        margin: 0;
    }
</style>