<script lang="ts">
    import { link } from "svelte-spa-router";
    import type { Routine } from "../../../models/routine";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Card from "../../display/card.svelte";

    export let routine: Routine = null;
</script>

{#if routine}
    <Card>
        <div class="title">
            <div />
            <p class="name large-text">{routine.name}</p>
            <Kebabmenu>
                <a href="/test">start</a>
                <a href={`/edit-routine/${routine.id}`} use:link>edit</a>
                <a href={`/history-routine/${routine.id}`}>history</a>
            </Kebabmenu>
        </div>
        <p class="small-text center-text">{routine.days.join(",")}</p>
        <div class="info">
            <div>
                <p class="small-text">last completed:</p>
                <p class="small-text">{new Date(routine.last_completed).toLocaleDateString()}</p>
            </div>
            <div>
                {#each routine.workouts as workout}
                    <p class="small-text">{workout.name}</p>
                {/each}
            </div>
        </div>
    </Card>
{/if}

<style>
    .title {
        display: grid;
        grid: 1fr / 1fr auto 1fr;
        place-items: center;
    }

    .name {
        margin: 0.5em;
        text-align: center;
    }

    .info {
        display: grid;
        grid: 1fr / 1fr 1fr;
    }

    p {
        margin: 0;
        padding: 0;
    }
</style>