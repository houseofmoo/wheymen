<script lang="ts">
    import { link, push } from "svelte-spa-router";
    import { UserStore } from "../../../stores/user-store";
    import { RequestTarget } from "../../../api/request-target";
    import { startSession } from "../../../api/session";
    import type { Routine } from "../../../models/routine";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Card from "../../display/card.svelte";
    import DeleteModal from "./delete-modal.svelte";
    import { Alert } from "../../../stores/alert-store";

    export let routine: Routine = null;
    let showModal;

    async function start(routine_id: string) {
        let session_res = await startSession($UserStore, routine_id);
        if (session_res.status_code === 200) {
            push(`/make-gains/${session_res.result.id}`);
            console.log('going to new page');
            return;
        } else {
            Alert.setMsg("Unable to start routine");
        }
    }
</script>

{#if routine}
    <Card>
        <div class="title">
            <div />
            <p class="name large-text center-text">{routine.name}</p>
            <Kebabmenu>
                <button class="link-button" on:click={() => start(routine.id)}>start</button>
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