<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    import { flip } from 'svelte/animate';
    import { UserStore } from "../../../stores/user-store";
    import { updateWorkout } from "../../../api/workout";
    import type { SessionSet, SessionWorkout } from "../../../models/session";
    import { RestStartTime } from "../../../stores/session-time";
    import Card from "../../display/card.svelte";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import HistoryModal from "./history-modal.svelte";

    export let workout: SessionWorkout = null;
    let history: SessionWorkout[] = [];
    const dispatch = createEventDispatcher();
    const SET_CHANGED = "set-changed";

    let showHistory: () => void;
    let hideKebab;

    onMount(async () => {
        getWorkoutHistory();

        // PROBLEM
        // a session is stored in the database with the sets list
        // when a session is "started" the sets list is empty so we populate it from the history
        // when a session is "resumed" the sets list is already populated, so we do not want to edit it

        // if sets are empty
        if (workout.sets.length <= 0) {
            // use recent history to populate sets
            if (history.length > 0) {
                let recent = history[0];
                for (let i = 0; i < recent.sets.length; i++) {
                    workout.sets = [...workout.sets, 
                        {
                            weight: recent.sets[i].weight,
                            reps: recent.sets[i].reps,
                            complete: false,
                        }
                    ];
                }
            // populate with dummy data and let user fill in the rest
            } else {
                workout.sets = [
                    {
                        weight: 0,
                        reps: 0,
                        complete: false,
                    }
                ]
            }
        }
    });

    function getWorkoutHistory() {
        // todo: get this workouts history
        // do NOT show the loading screen
    }

    function addSet() {
        workout.sets = [...workout.sets, {
            weight: 0,
            reps: 0,
            complete: false,
        }];
        hideKebab();
        dispatch(SET_CHANGED);
    }

    function removeSet() {
        const index = workout.sets.length - 1;
        if (index <= 0) {
            workout.sets = [];
        } else {
            workout.sets.splice(index, 1);
            workout.sets = [...workout.sets];
        }
        hideKebab();
        dispatch(SET_CHANGED);
    }

    function setCompleteToggle(set: SessionSet) {
        set.complete = !set.complete;
        RestStartTime.reset();
        dispatch(SET_CHANGED);
    }

    async function update_workout() {
        let w = {
            id: workout.workout_id,
            user_id: $UserStore.id,
            name: workout.workout_name,
            category: workout.workout_category,
            note: workout.workout_note,
        };
        await updateWorkout($UserStore, w, null, null);
    }
</script>

<Card>
    <div class="workout-title">
        <div />
        <p class="large-text">{workout.workout_name}</p>
        <Kebabmenu bind:hide={hideKebab}>
            <button class="link-button" on:click={addSet}>add set</button>
            <button class="link-button" on:click={removeSet}>remove set</button>
            <button class="link-button" on:click={showHistory}>view history</button>
        </Kebabmenu>
    </div>
    <textarea class="note" bind:value={workout.workout_note} on:change={update_workout} />
    <div class="set">
        <p class="small-text margin-0 padding-0">weight</p>
        <p class="small-text margin-0 padding-0">reps</p>
        <div class="margin-0 padding-0" />
    </div>
    {#each workout.sets as set, i (i)}
        <div class="set" class:complete={set.complete} transition:fade|local="{{duration: 150}}" animate:flip|local="{{duration: 200}}">
            <input type="number" 
                class="styled-input wide-100 center-text"
                class:invalid={set.weight === null}
                bind:value={set.weight}
                disabled={set.complete} />
            <input type="number" 
                class="styled-input wide-100 center-text" 
                class:invalid={set.reps === null}
                bind:value={set.reps}
                disabled={set.complete} />
                <button class="check-button" on:click={() => setCompleteToggle(set)}>&check;</button>
        </div>
    {/each}
</Card>

<HistoryModal history={history} bind:showHistory={showHistory} />

<style>
    .note {
        width: 100%;
        height: 50px;
        padding: 0.5em;
        box-sizing: border-box;
        border: 1px solid black;
        background-color: var(--primary-color-800);
        font-size: var(--small-font-size);
        resize: none;
        margin-bottom: 1em;
    }

    .workout-title {
        display: grid;
        grid: 1fr / 1fr auto 1fr;
        place-items: center;
    }

    .set {
        display: grid;
        grid: 1fr / repeat(2, 2fr) 1fr;
        grid-gap: 1em;
        width: 100%;
        place-content: center;
        place-items: center;
        margin-bottom: 1em;
    }

    .check-button {
        font-size: 1.5em;
        margin: 0;
        padding: 0;
        border: none;
        cursor: pointer;
        text-align: right;
        color: var(--text-color);
    }

    .complete > input  {
        color: var(--secondary-color-faded);
        border-bottom: solid 1px var(--secondary-color-faded);
    }

    .complete > button {
        color: var(--secondary-color-faded);
    }

    .invalid {
        color: red;
        border-bottom: 1px solid red;
    }

    input[type=number] { 
        -moz-appearance: textfield;
        appearance: textfield;
        margin: 0; 
    }

    input[type=number]::-webkit-inner-spin-button,
    input[type=number]::-webkit-outer-spin-button { 
        -webkit-appearance: none; 
        margin: 0; 
    }
</style>