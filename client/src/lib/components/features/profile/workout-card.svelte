<script lang="ts">
    import { link } from "svelte-spa-router";
    import { RequestTarget } from "../../../api/urls";
    import type { Workout } from "../../../models/workout";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Card from "../../display/card.svelte";
    import DeleteModal from "./delete-modal.svelte";

    export let workout: Workout = null;
    let showModal;

</script>

{#if workout}
    <Card>
        <div class="title">
            <div />
            <p class="name large-text center-text">{workout.name}</p>
            <Kebabmenu>
                <a href={`/edit-workout/${workout.id}`} use:link>edit</a>
                <a href="/test">history</a>
                <button class="link-button" on:click={() => showModal()}>delete</button>
            </Kebabmenu>
        </div>
        <div class="info">
            <p class="category margin-0 padding-0">{workout.category}</p>
        </div>
    </Card>
{/if}

<DeleteModal 
    item={workout} 
    target={RequestTarget.DeleteWorkout} 
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