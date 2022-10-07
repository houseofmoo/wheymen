<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import { continueRoutine } from "../api/routine";
    import type { Routine } from "../models/routine";
    import MakeGains from "../components/features/gains/make-gains.svelte";
    import type { InProgress } from "../models/in-progress";

    export let params = { id: null };
    let routine_id = params.id;
    let in_progress: InProgress = null;

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
        }

        // NOTE: when we land here we expect to just have an inprogress workout object
        // let a page level component create that and pass it to us
        
        //let routine_id = "routines:olisz8rlthi3ux7b7aa5";
        const resp = await continueRoutine(routine_id, $UserStore);
       
        // convert into a valid format


    });
</script>

{#if $UserStore && in_progress}
    <div>
        <MakeGains in_progress={in_progress} />
    </div>
{/if}

<style>
  
</style>