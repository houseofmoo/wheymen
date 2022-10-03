<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import { getRoutine } from "../api/routine";
    import type { Routine } from "../models/routine";
    import Card from "../components/display/card.svelte";
    import Kebabmenu from "../components/display/kebab-menu.svelte";
    import Title from "../components/display/title.svelte";

    export let params = { id: null };
    let routine_id = params.id;
    let routine: Routine = null;

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
        }

        // NOTE: when we land here we either are starting a new workout
        // or continuing a workout that was in progress...
        // need to handle both
        
        //let routine_id = "routines:olisz8rlthi3ux7b7aa5";

        const resp = await getRoutine(routine_id, $UserStore);
        if (resp.count > 0) {
            routine = resp.result;
        } else {
            // TODO: maybe no routine, maybe error occured
        }

        // get histories of workouts to get set/weight counts

        // store the workout in "workout in progress" table
    });
</script>

{#if $UserStore && routine}
    <div>
        <Title subtitle={"gains"} />
        <div>
            <p class="largest-text center-text">{routine.name}</p>
            <Kebabmenu>
                <button class="link-button" on:click={() => console.log("")}>sort workouts</button>
                <button class="link-button" on:click={() => console.log("")}>add workout</button>
                <button class="link-button" on:click={() => console.log("")}>create workout</button>
            </Kebabmenu>
            <p>total time</p>
            <p>cancel routine</p>
            <p>modify workoutlist -> order, create, select, or remove a workout. probably takes us to a different page after we store the current progress</p>
            <p>timer that counts time since last set (sticky)</p>
            <p>update inprogress workout db table once per min and/or everytime a set is completed</p>
        </div>
        {#each routine.workouts as workout}
            <Card>
                <div class="workout-title">
                    <div />
                    <p class="large-text">{workout.name}</p>
                    <Kebabmenu>
                        <button class="link-button" on:click={() => console.log(workout.note)}>add set</button>
                        <button class="link-button" on:click={() => console.log(workout.note)}>history</button>
                        <button class="link-button" on:click={() => console.log(workout.note)}>skip</button>
                    </Kebabmenu>
                </div>
                <textarea class="note" bind:value={workout.note} />
                <div class="workout-grid">
                    <div class="set">
                        <p class="small-text margin-0 padding-0">weight</p>
                        <p class="small-text margin-0 padding-0">reps</p>
                        <div class="margin-0 padding-0" />

                        <input class="styled-input wide-100" />
                        <input class="styled-input wide-100" />
                        <button class="link-button">x</button>

                        <input class="styled-input wide-100" />
                        <input class="styled-input wide-100" />
                        <button class="link-button">x</button>

                        <input class="styled-input wide-100" />
                        <input class="styled-input wide-100" />
                        <button class="link-button">x</button>
                    </div>
                </div>
            </Card>
        {/each}
        <button class="wide-100 margin-0">complete workout</button>
    </div>
{/if}

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

    .workout-grid {
        display: grid;
        grid: 1fr / 1fr;
        grid-gap: 1em;
        width: 100%;
    }

    .set {
        display: grid;
        grid: 1fr / repeat(3, 1fr);
        grid-gap: 1em;
        width: 100%;
        place-content: center;
        place-items: center;
        margin-bottom: 1em;
    }
</style>