<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
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
        if (workout.id === null) {
            const workoutRes = await insertWorkout($UserStore, workout);
            if (workoutRes.result !== null) {
                workout = workoutRes.result;
                push("/profile");
            } else {
                // TODO: handle insert failed
                console.log("error inserting routine");
            }
        } else {
            const workoutRes = await updateWorkout($UserStore, workout);
            if (workoutRes.result !== null) {
                workout = workoutRes.result;
                push("/profile");
            } else {
                // TODO: handle update failed
                console.log("error inserting routine");
            }
        }

        if (selected_routines.length > 0) {
            // update routines in selected_routines since they now have
            // another workout in them
            // TODO: have to update any routine that exists in "selected_routines" since
            // this workout may have been added to it
        }
    }

    async function deleteThisWorkout() {
        // TODO: confirmation box
        
        if (workout.id === null) {
            push("/profile");
            return;
        }

        await deleteWorkout(workout.id, $UserStore);
        push("/profile");
        // TODO: handle delete failed
    }

    function addRoutine(e: any) {
        const routine = e.detail;
        // from unselected to selected
    }

    function removeRoutine(routine: Routine) {
        // from selected to unselected
    }
</script>

{#if workout}
    <div class="page">
        <div class="content">
            <input placeholder="Routine Name" bind:value={workout.name} />
            <CategorySelector bind:selected={workout.category} />
            <textarea placeholder="Notes" bind:value={workout.note} />
            <div class="action-buttons">
                <button on:click={saveWorkout}>save</button>
                <button on:click={() => push("/profile")}>cancel</button>
                <button on:click={deleteThisWorkout}>delete</button>
            </div>
            <RoutineSelectorModal bind:routines={unselected_routines} on:routine-selected={addRoutine} />
            {#each selected_routines as routine}
            <div class="routines">
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

    .routines {
        display: grid;
        grid: auto / 1fr auto;
        background-color: var(--lightgrey);
        border: 1px solid black;
        place-items: center;
        width: 100%;
        height: 4em;
        font-size: 0.9em;
    }

    .routines > p {
        width: 100%;
        font-size: 0.9em;
        margin-left: 2em;
    }
</style>