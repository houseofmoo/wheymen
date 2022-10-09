<script lang="ts">
    import { link } from "svelte-spa-router";
    import { UserStore } from "../../../stores/user-store";
    import { deleteSession } from "../../../api/session";
    import type { Session } from "../../../models/session";
    import Kebabmenu from "../../display/kebab-menu.svelte";
    import Card from "../../display/card.svelte";

    export let session: Session = null;

    async function cancelSession() {
        const res = await deleteSession($UserStore, session.id);
        if (res.status_code !== 200 && res.status_code !== 204) {
            // TODO: handle error?
        }
        session = null;
    }

</script>

{#if session}
    <Card>
        <div class="title">
            <p class="small-text">in-progress:</p>
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