<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { getRoutine } from "../api/routine";
    import { UserStore } from "../stores/user-store";
    import type { Routine } from "../models/routine";
    import RoutineEditor from "../components/features/routine-editor/routine-editor.svelte"
    import Title from "../components/display/title.svelte";

    export let params = { id: null };
    let routine_id = params.id;
    let routine: Routine = null;

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
            return;
        }
        
        const resp = await getRoutine(routine_id, $UserStore);
        if (resp.count > 0) {
            routine = resp.result;
        } else {
            // TODO: maybe no routine, maybe error occured
        }
    });
</script>

{#if $UserStore && routine}
    <div>
        <Title subtitle={"edit routine"} />
        <RoutineEditor 
            bind:routine={routine} /> 
    </div>
{/if}