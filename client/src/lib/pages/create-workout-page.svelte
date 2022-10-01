<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import { Category } from "../models/category";
    import type { Workout } from "../models/workout";
    import WorkoutEditor from "../components/features/workout-editor/workout-editor.svelte";
    import Title from "../components/display/title.svelte";

    let newWorkout: Workout = {
        id: null,
        user_id: null,
        name: "",
        category: Category.arms,
        note: "",
    };

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
            return;
        }

        // set new routine user id
        newWorkout.user_id = $UserStore.id;
    });
</script>

{#if $UserStore}
    <div class="page">
        <Title subtitle={"create workout"} />
        <WorkoutEditor 
            bind:workout={newWorkout} />
    </div> 
{/if}

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }
</style>