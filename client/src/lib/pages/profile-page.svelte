<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { getAllWorkouts } from "../api/workout";
    import { UserStore } from "../stores/user-store";
    import { getAllRoutines } from "../api/routine";
    import type { Routine } from "../models/routine";
    import type { Workout } from "../models/workout";
    import Title from "../components/display/title.svelte";
    import RoutineCard from "../components/features/profile/routine-card.svelte";
    import WorkoutCard from "../components/features/profile/workout-card.svelte";

    export let params = { tab: null };
    let routines: Routine[] = [];
    let workouts: Workout[] = [];
    
    type tabs = "routines" | "workouts" | "account";
    let current_tab: tabs = params && params.tab ? params.tab : "routines";

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
        }

        await getRoutines();
        await getWorkouts();
    });

    async function getRoutines() {
        const routine_res = await getAllRoutines($UserStore);
        if (routine_res.result !== null) {
            routines = routine_res.result;
        } else {
            // TODO: maybe no routines, maybe error occured
        }
    }

    async function getWorkouts() {
        const workout_res = await getAllWorkouts($UserStore);
        if (workout_res.result !== null) {
            workouts = workout_res.result;
        } else {
            // TODO: maybe no workouts, maybe error occured
        }
    }

    async function getSessinos() {

    }

    async function refresh() {
        await getRoutines();
        await getWorkouts();
    }

    function changeTab(tab: "routines" | "workouts" | "account") {
        current_tab = tab;
    }
</script>

{#if $UserStore}
    <div>
        <Title subtitle={current_tab} />
        <div class="action-buttons-grid">
            <button class={current_tab === "routines" ? "action-button-selected" : "action-button"} on:click={() => changeTab("routines")}>routines</button>
            <button class={current_tab === "workouts" ? "action-button-selected" : "action-button"} on:click={() => changeTab("workouts")}>workouts</button>
            <button class={current_tab === "account" ? "action-button-selected" : "action-button"} on:click={() => changeTab("account")}>account</button>
        </div>

        {#if current_tab === "routines"}
            <div>
                {#each routines as routine}
                    <RoutineCard {routine} on:item-deleted={refresh}/>
                {/each}
                <button class="wide-100 margin-0" on:click={() => push('/create-routine')}>create routine</button>
            </div>
        {:else if current_tab === "workouts"}
            <div>
                {#each workouts as workout}
                    <WorkoutCard {workout} on:item-deleted={refresh} />
                {/each}
                <button class="wide-100 margin-0" on:click={() => push('/create-workout')}>create workout</button>
            </div>
        {:else}
            <div>
                <p style="text-align: center">{$UserStore.email}</p>
                <button class="wide-100 margin-0" on:click={() => push('/logout')}>logout</button>
            </div>
        {/if}
    </div>
{/if}

<style>
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
        background-color: var(--primary-color-800);
        width: 100%;
        padding: 0;
        margin: 0;
        border: 0;
    }

    .action-button-selected {
        font-family: 'Space Grotesk', sans-serif;
        background-color: var(--primary-color-800);
        width: 100%;
        padding: 0;
        margin: 0;
        border: 0;
        text-decoration: underline;
    }
</style>