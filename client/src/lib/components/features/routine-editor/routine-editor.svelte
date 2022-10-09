<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { fade } from "svelte/transition";
    import { flip } from 'svelte/animate';
    import type { Routine } from "../../../models/routine";
    import type { Workout } from "../../../models/workout";
    import { insertRoutine, updateRoutine, deleteRoutine } from "../../../api/routine";
    import { getAllWorkouts } from "../../../api/workout";
    import { UserStore } from "../../../stores/user-store";
    import { Alert } from "../../../stores/alert-store";
    import { getUnrelatedWorkouts } from "../../../api/workout";
    import DaySelector from "./day-selector.svelte";
    import WorkoutSelectorModal from "./workout-selector-modal.svelte";
    import Remove from "../../display/icons/remove.svelte";
    import UpArrow from "../../display/icons/up-arrow.svelte";
    import DownArrow from "../../display/icons/down-arrow.svelte";
    import IconButton from "../../display/icon-button.svelte";
    

    export let routine: Routine;
    let unselected_workouts: Workout[] = [];

    onMount(async () => {
        let res = null;
        if (routine && routine.id) {
            res = await getUnrelatedWorkouts(routine.id, $UserStore);
        } else {
            res = await getAllWorkouts($UserStore);
        }

        if (res.status_code === 200 || res.status_code === 204) {
            unselected_workouts = res.result;
        } else {
            Alert.setMsg(`Encountered a problem fetching workouts: ${res.status_msg}`);
        }
    });

    async function saveRoutine() {
        let routine_res = null;

        if (routine.id === null) {
            routine_res = await insertRoutine($UserStore, routine);
        } else {
            routine_res = await updateRoutine($UserStore, routine);
        }

        if (routine_res.status_code === 200) {
            routine = routine_res.result;
            push("/profile/routines");
            return;
        }
        Alert.setMsg(`Unable to save Routine: ${routine_res.status_msg}`);
    }

    function addWorkout(e: any) {
        const workout = e.detail;
        routine.workouts = [...routine.workouts, workout];
        const filtered = unselected_workouts.filter((x) => x.id !== workout.id);
        unselected_workouts = [...filtered];
    }

    function removeWorkout(workout: Workout) {
        unselected_workouts = [...unselected_workouts, workout];
        const filtered = routine.workouts.filter((x) => x.id !== workout.id);
        routine.workouts = [...filtered];
    }

    function shift(from: number, to: number) {
        if (to === -1 || to >= routine.workouts.length) {
            return;
        }

        const fromWorkout = routine.workouts[from];
		const toWorkout = routine.workouts[to];

		routine.workouts[to] = fromWorkout;
		routine.workouts[from] = toWorkout;
    }
</script>

{#if routine}
    <div class="page">
        <div class="content">
            <input class="styled-input wide-95 large-text" placeholder="Routine name" bind:value={routine.name} />
            <DaySelector bind:days={routine.days} />
            <textarea class="styled-textarea" placeholder="Routine notes" bind:value={routine.note} />
            <div class="action-buttons">
                <button class="wide-100" on:click={() => push("/profile/routines")}>cancel</button>
                <button class="wide-100" on:click={saveRoutine}>save</button>
            </div>
            <WorkoutSelectorModal bind:workouts={unselected_workouts} on:workout-selected={addWorkout} />
            {#each routine.workouts as workout, i (workout.id)}
                <div class="workouts" transition:fade|local="{{duration: 150}}" animate:flip|local="{{duration: 200}}">
                    <p>{workout.name}</p>
                    <IconButton icon={DownArrow} on:click={() => shift(i, i+1)} />
                    <IconButton icon={UpArrow} on:click={() => shift(i, i-1)} />
                    <IconButton icon={Remove} on:click={() => removeWorkout(workout)} />
                </div>
            {/each}
        </div>
    </div>
{/if}

<style>
    .page {
        width: 100%;
        margin: 2em 0 0 0;
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

    .action-buttons {
        display: grid;
        grid: auto / 1fr 1fr;
        grid-gap: 1em;
        place-items: center;
        place-content: center;
        width: 100%;
    }

    .workouts {
        display: grid;
        grid: auto / 2fr repeat(3, 1fr);
        background-color: var(--primary-color-400);
        border: 1px solid black;
        place-items: center;
        width: 100%;
        height: 4em;
        font-size: var(--small-font-size);
    }

    .workouts > p {
        width: 100%;
        font-size: var(--largest-font-size);
        margin-left: 2em;
    }
</style>
