<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { getRoutine } from "../api/routine";
    import { UserStore } from "../stores/user-store";
    import { Alert } from "../stores/alert-store";
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

        // if there is no id, leave
        if (routine_id) {
            push('/profile/routines');
            return;
        }
        
        const res = await getRoutine(routine_id, $UserStore);
        if (res.status_code !== 200) {
            Alert.setMsg(`Routine does not exist, returning to profile: ${res.status_msg}`);
            push('/profile/routines');
            return;
        }

        routine = res.result;
    });
</script>

{#if $UserStore && routine}
    <div>
        <Title subtitle={"edit routine"} />
        <RoutineEditor 
            bind:routine={routine} /> 
    </div>
{/if}