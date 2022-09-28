<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import type { Routine } from "../../../models/routine";
    import type { Workout } from "../../../models/workout";
    import { insertRoutine, updateRoutine, deleteRoutine } from "../../../api/routine";
    import { getAllWorkouts } from "../../../api/workout";
    import { UserStore } from "../../../stores/user-store";
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
        if (routine && routine.id) {
            const resp = await getUnrelatedWorkouts(routine.id, $UserStore);
            if (resp.result !== null) {
                unselected_workouts = resp.result;
            }
        } else {
            const resp = await getAllWorkouts($UserStore);
            if (resp.result !== null) {
                unselected_workouts = resp.result;
            }
        }
    });

    async function saveRoutine() {
        if (routine.id === null) {
            const routineRes = await insertRoutine($UserStore, routine);
            if (routineRes.result !== null) {
                routine = routineRes.result;
                push("/profile");
            } else {
                // TODO: handle insert failed
                console.log("error inserting routine");
            }
        } else {
            const routineRes = await updateRoutine($UserStore, routine);
            if (routineRes.result !== null) {
                routine = routineRes.result;
                push("/profile");
            } else {
                // TODO: handle update failed
                console.log("error inserting routine");
            }
        }
    }

    async function deleteThisRoutine() {
        // TODO: confirmation box
        
        if (routine.id === null) {
            push("/profile");
            return;
        }

        await deleteRoutine(routine.id, $UserStore);
        push("/profile");
        // TODO: handle delete failed
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
            <input placeholder="Routine Name" bind:value={routine.name} />
            <DaySelector bind:days={routine.days} />
            <textarea placeholder="Notes" bind:value={routine.note} />
            <div class="action-buttons">
                <button on:click={saveRoutine}>save</button>
                <button on:click={() => push("/profile")}>cancel</button>
                <button on:click={deleteThisRoutine}>delete</button>
            </div>
            <WorkoutSelectorModal bind:workouts={unselected_workouts} on:workout-selected={addWorkout} />
            {#each routine.workouts as workout, i}
                <div class="workouts">
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

    .action-buttons {
        display: grid;
        grid: auto / repeat(3, 1fr);
        grid-gap: 1em;
        width: 100%;
    }

    .action-buttons > button {
        width: 100%;
    }

    .workouts {
        display: grid;
        grid: auto / 2fr repeat(3, 1fr);
        background-color: var(--lightgrey);
        border: 1px solid black;
        place-items: center;
        width: 100%;
        height: 4em;
        font-size: 0.9em;
    }

    .workouts > p {
        width: 100%;
        font-size: 0.9em;
        margin-left: 2em;
    }
</style>
