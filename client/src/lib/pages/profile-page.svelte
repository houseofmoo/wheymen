<script lang="ts">
    import { onMount } from "svelte";
    import { push, link } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import { getAllRoutines } from "../api/routine";
    import type { Routine } from "../models/routine";
    import RoutineCard from "../components/routine-card.svelte";
    import Title from "../components/display/title.svelte";

    let routines: Routine[] = [];

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
        }

        // get users routines
        const resp = await getAllRoutines($UserStore);
        if (resp.count > 0) {
            routines = resp.result;
        } else {
            // TODO: maybe no routines, maybe error occured
        }
    });

 
</script>

{#if $UserStore}
    <div class="page">
        <Title subtitle={"profile"} />
        <div class="action-buttons">
            <button on:click={() => push('/account')}>account</button>
            <button on:click={() => push('/create-routine')}>+routine</button>
            <button on:click={() => push('/create-workout')}>+workout</button>
        </div>
        {#each routines as routine}
            <RoutineCard routine={routine} />
        {/each}
    </div>
{/if}

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }

    .action-buttons {
        display: grid;
        grid: 1fr / repeat(3, 1fr);
        place-content: center;
        place-items: center;
        grid-gap: 1em;
    }

    .action-buttons > button {
        width: 100%;
        margin: 0;
    }
</style>