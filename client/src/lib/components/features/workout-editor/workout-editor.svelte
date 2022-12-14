<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { fade } from "svelte/transition";
    import { flip } from 'svelte/animate';
    import { getAllRoutines } from "../../../api/routine";
    import type { Workout } from "../../../models/workout";
    import type { Routine } from "../../../models/routine";
    import { UserStore } from "../../../stores/user-store";
    import { Alert } from "../../../stores/alert-store";
    import CategorySelector from "./category-selector.svelte";
    import { insertWorkout, updateWorkout, deleteWorkout } from "../../../api/workout";
    import RoutineSelectorModal from "./routine-selector-modal.svelte";
    import Remove from "../../display/icons/remove.svelte";
    import IconButton from "../../display/icon-button.svelte";    
    import UpArrow from "../../display/icons/up-arrow.svelte";
    import DownArrow from "../../display/icons/down-arrow.svelte";

    export let workout: Workout;
    let selected_routines: Routine[] = [];
    let unselected_routines: Routine[] = [];

    onMount(async () => {
        if (workout) {
            const res = await getAllRoutines($UserStore);
            let routines: Routine[] = [];
            if (res.status_code === 200 || res.status_code === 204) {
                routines = res.result;
            } else {
                Alert.setMsg(`Encountered a problem fetching routines: ${res.status_msg}`);
            }


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
        let workout_res = null;

        if (workout.id === null) {
            workout_res = await insertWorkout($UserStore, workout, selected_ids, unseleced_ids);
        } else {
            workout_res = await updateWorkout($UserStore, workout, selected_ids, unseleced_ids);
        }

        if (workout_res.status_code === 200) {
            workout = workout_res.result;
            push("/profile/workouts");
            return;
        }
        Alert.setMsg(`Unable to save Workout: ${workout_res.status_msg}`);
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


    function shift(from: number, to: number) {}
</script>

{#if workout}
    <div class="page">
        <div class="content">
            <input class="styled-input wide-95 large-text" placeholder="Workout name" bind:value={workout.name} />
            <CategorySelector bind:selected={workout.category} />
            <textarea class="styled-textarea" placeholder="Workout notes" bind:value={workout.note} />
            <div class="action-buttons">
                <button class="wide-100"  on:click={() => push("/profile/workouts")}>cancel</button>
                <button class="wide-100"  on:click={saveWorkout}>save</button>
            </div>
            <RoutineSelectorModal bind:routines={unselected_routines} on:routine-selected={addRoutine} />
            {#each selected_routines as routine, i (routine.id)}
            <div class="routines" transition:fade|local="{{duration: 150}}" animate:flip|local="{{duration: 200}}">
                <p>{routine.name}</p>                 
                <IconButton icon={DownArrow} on:click={() => shift(i, i+1)} />
                <IconButton icon={UpArrow} on:click={() => shift(i, i-1)} />
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
        grid: auto / 2fr repeat(3, 1fr);
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