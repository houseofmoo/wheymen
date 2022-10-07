<script lang="ts">
    import { link } from "svelte-spa-router";
    import { RequestTarget } from "../../../api/urls";
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
        <div class="info">
            <p class="small-text center-text">{routine.days.join(", ")}</p>
            <p class="small-text center-text">{new Date(routine.last_completed).toLocaleDateString()}</p>
            <p class="small-text">{routine.note}</p>
        </div>
        <p class="center-text">{routine.workouts.map(x => x.name).join(" - ")}</p>
    </Card>
{/if}

<DeleteModal 
    item={routine} 
    target={RequestTarget.DeleteRoutine} 
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
        grid: 1fr / repeat(3, 1fr);
        place-content: center;
    }
</style>