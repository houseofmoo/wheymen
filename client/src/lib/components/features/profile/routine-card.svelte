<script lang="ts">
    import { link } from "svelte-spa-router";
    import { RequestPath } from "../../../api/shared";
    import type { Routine } from "../../../models/routine";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Card from "../../display/card.svelte";
    import DeleteModal from "./delete-modal.svelte";

    export let routine: Routine = null;
    let showModal;

</script>

{#if routine}
    <Card>
        <div class="title">
            <div />
            <p class="name large-text center-text">{routine.name}</p>
            <Kebabmenu>
                <a href={`/gains/${routine.id}`} use:link>start</a>
                <a href={`/edit-routine/${routine.id}`} use:link>edit</a>
                <a href={`/history-routine/${routine.id}`} use:link>history</a>
                <button class="link-button" on:click={() => showModal()}>delete</button>
            </Kebabmenu>
        </div>
        <p class="small-text center-text margin-0 padding-0">{routine.days.join(",")}</p>
        <div class="info">
            <div>
                <p class="small-text margin-0 padding-0">last completed:</p>
                <p class="small-text margin-0 padding-0">{new Date(routine.last_completed).toLocaleDateString()}</p>
            </div>
            <div>
                {#each routine.workouts as workout}
                    <p class="small-text margin-0 padding-0">{workout.name}</p>
                {/each}
            </div>
        </div>
    </Card>
{/if}

<DeleteModal 
    item={routine} 
    url={RequestPath.DeleteRoutine} 
    bind:show={showModal} 
    on:item-deleted />

<style>
    .title {
        display: grid;
        grid: 1fr / 1fr auto 1fr;
        place-items: center;
    }

    .name {
        margin: 0.5em;
    }

    .info {
        display: grid;
        grid: 1fr / 1fr 1fr;
    }
</style>