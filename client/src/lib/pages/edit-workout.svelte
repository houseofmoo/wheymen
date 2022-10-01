<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { getWorkout } from "../api/workout";
    import { UserStore } from "../stores/user-store";
    import type { Workout } from "../models/workout";
    import WorkoutEditor from "../components/features/workout-editor/workout-editor.svelte"
    import Title from "../components/display/title.svelte";

    export let params = { id: null };
    let workout_id = params.id;
    let workout: Workout = null;

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
            return;
        }

        const resp = await getWorkout(workout_id, $UserStore);
        if (resp.count > 0) {
            workout = resp.result;
        } else {
            // TODO: maybe no workout, maybe error occured
        }
    });
</script>

{#if $UserStore && workout}
    <div class="page">
        <Title subtitle={"edit workout"} />
        <WorkoutEditor 
            bind:workout={workout} /> 
    </div>
{/if}

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }
</style>