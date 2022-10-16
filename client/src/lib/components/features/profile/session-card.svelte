<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { link } from "svelte-spa-router";
    import { UserStore } from "../../../stores/user-store";
    import { Alert } from "../../../stores/alert-store";
    import { deleteSession } from "../../../api/session";
    import type { Session } from "../../../models/session";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Card from "../../display/card.svelte";

    export let session: Session = null;
    const dispatch = createEventDispatcher();

    async function cancelSession() {
        const res = await deleteSession($UserStore, session.id);
        if (res.status_code !== 200 && res.status_code !== 204) {
            Alert.setMsg(`Encountered a problem deleting session: ${res.status_msg}`);
            return;
        }
        dispatch('item-deleted');
        session = null;
    }
</script>

{#if session}
    <Card>
        <div class="title">
            <p class="small-text">unfinished:</p>
            <p class="name large-text center-text">{session.routine_name}</p>
            <Kebabmenu>
                <a href={`/make-gains/${session.id}`} use:link>continue</a>
                <button class="link-button" on:click={() => cancelSession()}>cancel</button>
            </Kebabmenu>
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
    }
</style>