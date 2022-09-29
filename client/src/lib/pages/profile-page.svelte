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
    let subtitle = "routines";
    let current_tab: "routines" | "workouts" | "account" = "routines";

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

        // TEST CODE TO BE REMOVED or moved to another tab on the profile page?
        const workout_res = await getAllWorkouts($UserStore);
        if (workout_res.count > 0) {
            workouts = workout_res.result;
        } else {
            // TODO: maybe no workouts, maybe error occured
        }
    });

    function changeTab(tab: "routines" | "workouts" | "account") {
        current_tab = tab;
        subtitle = tab;
    }
</script>

{#if $UserStore}
    <div class="page">
        <Title {subtitle} />
        <div class="action-buttons-grid">
            <button class={current_tab === "routines" ? "action-button-selected" : "action-button"} on:click={() => changeTab("routines")}>routines</button>
            <button class={current_tab === "workouts" ? "action-button-selected" : "action-button"} on:click={() => changeTab("workouts")}>workouts</button>
            <button class={current_tab === "account" ? "action-button-selected" : "action-button"} on:click={() => changeTab("account")}>account</button>
        </div>

        {#if current_tab === "routines"}
            <div>
                {#each routines as routine}
                    <RoutineCard routine={routine} />
                {/each}
                <button class="create-button" on:click={() => push('/create-routine')}>create routine</button>
            </div>
        {:else if current_tab === "workouts"}
            <div>
                {#each workouts as workout}
                    <button class="workout-button" on:click={() => push(`/edit-workout/${workout.id}`)}>{workout.name}</button>
                {/each}
                <button class="create-button" on:click={() => push('/create-workout')}>create workout</button>
                
            </div>
        {:else}
            <div>
                <p style="text-align: center">{$UserStore.email}</p>
                <button class="create-button" on:click={() => push('/logout')}>logout</button>
            </div>
        {/if}

    </div>
{/if}

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }

    .action-buttons-grid {
        display: grid;
        grid: 1fr / repeat(3, 1fr);
        place-content: center;
        place-items: center;
        grid-gap: 1em;
        margin-top: -0.5em;
        margin-bottom: 1em;
    }

    .action-button {
        font-family: 'Space Grotesk', sans-serif;
        background-color: var(--darkgrey);
        width: 100%;
        padding: 0;
        margin: 0;
        border: 0;
    }

    .action-button-selected {
        font-family: 'Space Grotesk', sans-serif;
        background-color: var(--darkgrey);
        width: 100%;
        padding: 0;
        margin: 0;
        border: 0;
        color: var(--orange);
    }

    .create-button {
        font-family: 'Space Grotesk', sans-serif;
        width: 100%;
        color: var(--orange);
        margin: 0;
    }

    .workout-button {
        width: 100%;
        margin: 0.25em 0;
    }
</style>