<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import { startSession } from "../api/session";
    import MakeGains from "../components/features/gains/make-gains.svelte";
    import type { Session } from "../models/session";

    export let params = { id: null };
    let routine_id = params.id;
    let session: Session = null;

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
        }

        const sessionRes = await startSession($UserStore, routine_id);
        if (sessionRes.result === null) {
            // TODO: alter error and probably leave?
        }
       
        session = sessionRes.result;
        console.log(session);
    });
</script>

{#if $UserStore && session}
    <div>
        <MakeGains session={session} />
    </div>
{/if}

<style>
  
</style>