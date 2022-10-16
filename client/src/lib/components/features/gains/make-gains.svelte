<script lang="ts">
    import { onMount } from "svelte";
    import { UserStore } from "../../../stores/user-store";
    import { updateSession } from "../../../api/session";
    import type { Session } from "../../../models/session";
    import { RestElapsed, SessionLength, SessionElapsed } from "../../../stores/session-time";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Title from "../../display/title.svelte";
    import StickyHeader from "./sticky-header.svelte";
    import SessionWorkoutCard from "./session-workout-card.svelte";
    import StopWatch from "../../display/icons/stop-watch.svelte"

    export let session: Session = null;
    let timeout;

    onMount(() => {
        SessionLength.setTimeSinceStart(session.duration_in_sec * 1000);
    });

    function convertTo(time: number) {
        let totalSeconds = Math.round(time / 1000);
		let sec = totalSeconds % 60;
		let min = Math.floor(totalSeconds / 60);
		if (sec < 0) sec = 0;
		if (min < 0) min = 0;
		return (min < 10 ? '0' + min : min) + ':' + (sec < 10 ? '0' + sec : sec);
    }

    async function update_session() {
        session.duration_in_sec = Math.round($SessionElapsed / 1000);
        session.workouts.forEach(w => {
            w.sets.forEach(s => {
                s.reps = Number(s.reps);
                s.weight = Number(s.weight);
            })
        })

        // delay updating for 1 second so we dont spam ourselves
        clearTimeout(timeout);
        timeout = setTimeout(async () => {
            const res = await updateSession($UserStore, session);
            if (res.status_code === 200) {
                session = res.result;
            }
        }, 1000)
    }

    function update() {
        let timeout;

        return () => {
            clearTimeout(timeout);
            timeout = setTimeout(() => {
                
            })
        }
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
                        <p class="small-text center-text">total: {convertTo($SessionElapsed)}</p>
                        <p class="small-text center-text"><StopWatch /> {convertTo($RestElapsed)}</p>
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
            <SessionWorkoutCard bind:workout={workout} on:set-changed={update_session} />
        {/each}
        <button class="wide-100 margin-0">complete session</button>
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
        scrollbar-width: none;  /* Firefox */
    }
    
    .overflow-container::-webkit-scrollbar { /* Hide scrollbar for Chrome, Safari and Opera */
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
</style>