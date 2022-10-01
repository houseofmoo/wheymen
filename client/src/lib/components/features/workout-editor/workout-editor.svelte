<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { fade } from "svelte/transition";
    import { flip } from 'svelte/animate';
    import { getAllRoutines } from "../../../api/routine";
    import type { Workout } from "../../../models/workout";
    import type { Routine } from "../../../models/routine";
    import { UserStore } from "../../../stores/user-store";
    import CategorySelector from "./category-selector.svelte";
    import { insertWorkout, updateWorkout, deleteWorkout } from "../../../api/workout";
    import RoutineSelectorModal from "./routine-selector-modal.svelte";
    import Remove from "../../display/icons/remove.svelte";
    import IconButton from "../../display/icon-button.svelte";

    export let workout: Workout;
    let selected_routines: Routine[] = [];
    let unselected_routines: Routine[] = [];

    onMount(async () => {
        if (workout) {
            const res = await getAllRoutines($UserStore);
            const routines = res.result;    // TODO: handle failure case

            if (routines && routines.length > 0) {
                // gather routines that contain the workout id
                for (let i = 0; i < routines.length; i++) {
                    for (let j = 0; j < routines[i].workouts.length; j++) {
                        if (routines[i].workouts[j].id === workout.id) {
                            selected_routines = [...selected_routines, routines[i]];
                        }
                    }
                }

                // unselected routines are what remains
                unselected_routines = routines.filter(r => !selected_routines.includes(r));
            }
        }
    });

    async function saveWorkout() {
        const selected_ids = selected_routines.map(x => x.id);
        const unseleced_ids = unselected_routines.map(x => x.id);
        
        if (workout.id === null) {
            const workoutRes = await insertWorkout($UserStore, workout, selected_ids, unseleced_ids);
            if (workoutRes.result !== null) {
                workout = workoutRes.result;
                push('/profile/workouts');
            } else {
                // TODO: handle insert failed
                console.log("error inserting workout");
            }
        } else {
            const workoutRes = await updateWorkout($UserStore, workout, selected_ids, unseleced_ids);
            if (workoutRes.result !== null) {
                workout = workoutRes.result;
                push('/profile/workouts');
            } else {
                // TODO: handle update failed
                console.log("error updating workout");
            }
        }
    }

    function addRoutine(e: any) {
        const routine = e.detail;
        selected_routines = [...selected_routines, routine];
        unselected_routines = unselected_routines.filter(r => r.id !== routine.id);
    }

    function removeRoutine(routine: Routine) {
        unselected_routines = [...unselected_routines, routine];
        selected_routines = selected_routines.filter(r => r.id !== routine.id);
    }
</script>

{#if workout}
    <div class="page">
        <div class="content">
            <input class="styled-input wide-95 large-text" placeholder="Workout name" bind:value={workout.name} />
            <CategorySelector bind:selected={workout.category} />
            <textarea class="styled-textarea" placeholder="Workout notes" bind:value={workout.note} />
            <div class="action-buttons">
                <button class="wide-100"  on:click={saveWorkout}>save</button>
                <button class="wide-100"  on:click={() => push("/profile/workouts")}>cancel</button>
            </div>
            <RoutineSelectorModal bind:routines={unselected_routines} on:routine-selected={addRoutine} />
            {#each selected_routines as routine (routine.id)}
            <div class="routines" transition:fade|local="{{duration: 150}}" animate:flip|local="{{duration: 200}}">
                <p>{routine.name}</p>
                <IconButton icon={Remove} on:click={() => removeRoutine(routine)} />
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

    .routines {
        display: grid;
        grid: auto / 1fr auto;
        background-color: var(--primary-color-400);
        border: 1px solid black;
        place-items: center;
        width: 100%;
        height: 4em;
        font-size: var(--small-font-size);
    }

    .routines > p {
        width: 100%;
        font-size: var(--largest-font-size);
        margin-left: 2em;
    }
</style>