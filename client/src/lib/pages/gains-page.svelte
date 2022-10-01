<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import { getRoutine } from "../api/routine";
    import type { Routine } from "../models/routine";

    export let params = { id: null };
    let routine_id = params.id;
    let routine: Routine = null;

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
        }

        const resp = await getRoutine(routine_id, $UserStore);
        if (resp.count > 0) {
            routine = resp.result;
        } else {
            // TODO: maybe no routine, maybe error occured
        }


        // we need the routine and the lifts which we're getting
        // but we also need lift histories so we can prepopulate the workout lists

    });
</script>

{#if $UserStore && routine}
    <div class="page">
        {routine.name}
    </div>
{/if}

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }

  
</style>