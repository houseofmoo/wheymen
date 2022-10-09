<script lang="ts">
    import { UserStore } from "../../../stores/user-store";
    import type { Session } from "../../../models/session";
    import { restElapsed, restTime, sessionElapsed } from "../../../stores/session-time";
    import Card from "../../display/card.svelte";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Title from "../../display/title.svelte";
    import StickyHeader from "./sticky-header.svelte";


    export let session: Session = null;

    // we need a timer that starts when we land on this page
    // we need to record the date/time we started
    // we need to get the history for each workout in the session

    // every time the user completes a set, update the session

    function convertTo(time: number) {
        let totalSeconds = Math.round(time / 1000);
		let sec = totalSeconds % 60;
		let min = Math.floor(totalSeconds / 60);
		if (sec < 0) sec = 0;
		if (min < 0) min = 0;
		return (min < 10 ? '0' + min : min) + ':' + (sec < 10 ? '0' + sec : sec);
    }

    function resetRestTimer() {
        restTime.reset()
    }

    function updateSession() {
        // everytime user finishes a set
    }

    function getWorkoutHistor() {
        // get the history for each workout that is part of this session
        // do that onMount 
    }
</script>

{#if $UserStore && session}
    <div>
        <Title subtitle={"gains"} />
        <StickyHeader show={true}>
            <div class="sticky-content">
                <div class="overflow-container">
                    <div class="overflow">
                        <p class="largest-text center-text">{session.routine_name}</p>
                        <p class="small-text center-text">total: {convertTo($sessionElapsed)}</p>
                        <p class="small-text center-text">rested: {convertTo($restElapsed)}</p>
                    </div>
                </div>
                <Kebabmenu>
                    <button class="link-button" on:click={() => console.log("")}>sort workouts</button>
                    <button class="link-button" on:click={() => console.log("")}>add workout</button>
                    <button class="link-button" on:click={() => console.log("")}>create workout</button>
                    <button class="link-button" on:click={() => console.log("")}>cancel session</button>
                </Kebabmenu>
            </div>
        </StickyHeader>
        {#each session.workouts as workout}
            <Card>
                <div class="workout-title">
                    <div />
                    <p class="large-text">{workout.workout_name}</p>
                    <Kebabmenu>
                        <button class="link-button" on:click={() => console.log("")}>add set</button>
                        <button class="link-button" on:click={() => console.log("")}>history</button>
                        <button class="link-button" on:click={() => console.log("")}>skip</button>
                        <button class="link-button" on:click={() => console.log("")}>remove</button>
                    </Kebabmenu>
                </div>
                <textarea class="note" bind:value={workout.workout_note} />
                <div class="sets">
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
            </Card>
        {/each}
        <button class="wide-100 margin-0">complete workout</button>
    </div>
{/if}

<style>
    .sticky-content {
        display: grid;
        grid: auto / auto 1fr;
        place-items: center;
        padding: 0.5em 1em;
    }

    .overflow-container {
        width: 100%;
        overflow-x: scroll;  
        ms-overflow-style: none;  /* IE and Edge */
        scrollbar-width: none;  /* Firefox */
    }

    /* Hide scrollbar for Chrome, Safari and Opera */
    .overflow-container::-webkit-scrollbar {
        display: none;
    }

    .overflow {
        display: flex;
        gap: 1em;
        place-items: center;
        width: fit-content;
    }

    .overflow > p {
        margin: 0;
        padding: 0;
        white-space: nowrap;
    }

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

    .sets {
        display: grid;
        grid: 1fr / repeat(2, 2fr) 1fr;
        grid-gap: 1em;
        width: 100%;
        place-content: center;
        place-items: center;
        margin-bottom: 1em;
    }
</style>