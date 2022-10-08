<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import type { Routine } from "../models/routine";
    import RoutineEditor from "../components/features/routine-editor/routine-editor.svelte"
    import Title from "../components/display/title.svelte";

    let newRoutine: Routine = {
        id: null,
        user_id: null,
        name: "",
        days: [],
        last_completed: new Date().toISOString(),
        note: "",
        workouts: [],
    };

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
            return;
        }

        // set new routine user id
        newRoutine.user_id = $UserStore.id;
    });
</script>

{#if $UserStore}
    <div>
        <Title subtitle={"create routine"} />
        <RoutineEditor 
            bind:routine={newRoutine} />
    </div>
{/if}
