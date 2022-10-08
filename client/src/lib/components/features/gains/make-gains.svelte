<script lang="ts">
    import { UserStore } from "../../../stores/user-store";
    import Card from "../../display/card.svelte";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Title from "../../display/title.svelte";
    import type { Session } from "../../../models/session";

    export let session: Session = null;
</script>

{#if $UserStore && session}
    <div>
        <Title subtitle={"gains"} />
        <div>
            <p class="largest-text center-text">{session.routine_name}</p>
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
        {#each session.workouts as workout}
            <Card>
                <div class="workout-title">
                    <div />
                    <p class="large-text">{workout.workout_name}</p>
                    <Kebabmenu>
                        <button class="link-button" on:click={() => console.log("")}>add set</button>
                        <button class="link-button" on:click={() => console.log("")}>history</button>
                        <button class="link-button" on:click={() => console.log("")}>skip</button>
                    </Kebabmenu>
                </div>
                <textarea class="note" bind:value={workout.workout_note} />
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