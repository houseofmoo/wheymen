<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { getWorkout } from "../api/workout";
    import { UserStore } from "../stores/user-store";
    import { Alert } from "../stores/alert-store";
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

        // if there is no id, leave
        if (workout_id) {
            push('/profile/workouts');
            return;
        }

        const res = await getWorkout(workout_id, $UserStore);
        if (res.status_code !== 200) {
            Alert.setMsg(`Workout does not exist, returning to profile: ${res.status_msg}`);
            push('/profile/workouts');
            return;
        }

        workout = res.result;
    });
</script>

{#if $UserStore && workout}
    <div>
        <Title subtitle={"edit workout"} />
        <WorkoutEditor 
            bind:workout={workout} /> 
    </div>
{/if}