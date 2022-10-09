<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import { startSession, doesSessionExists, updateSession } from "../api/session";
    import MakeGains from "../components/features/gains/make-gains.svelte";
    import type { Session } from "../models/session";
    import { Alert } from "../stores/alert-store";

    export let params = { id: null };
    let routine_id = params.id;
    let session: Session = null;

    let session_exists = false;

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
            return;
        }

        const session_exists_res = await doesSessionExists($UserStore, routine_id);
        if (session_exists_res.status_code === 200) {
            session_exists = true;
            session = session_exists_res.result;
        } else if (session_exists_res.status_code === 204) {
            session_exists = false;
            const session_res = await startSession($UserStore, routine_id);
            if (session_res.status_code === 200) {
                session = session_res.result;
            } else if (session_res.status_code === 204) {
                session_exists = true;
                session = session_res.result;
            } else {
                Alert.setMsg("Session was not able to be started, returning to proile");
                push('/profile/routines');
                return;
            }
        } else {
            Alert.setMsg("Session was not able to be started, returning to proile");
            push('/profile/routines');
            return;
        }
    });

    function continueSession() {
        push(`/continue-session/${session.id}`);
    }

    async function startNewSession() {
        session.start_time = new Date().toISOString();
        session.duration_in_sec = 0;
        session.workouts.map(w => w.sets = []);

        const session_res = await updateSession($UserStore, session);
            if (session_res.status_code === 200) {
                session = session_res.result;
            } else {
                Alert.setMsg("Session was not able to be started, returning to proile");
                push('/profile/routines');
                return;
            }
    }
</script>

{#if $UserStore && session}
    {#if session_exists}
        <p>A session using this routine is already in progress: </p>
        <p class="center-text">{session.routine_name}</p>
        <p class="center-text">Start on: {new Date(session.start_time).toLocaleDateString()}</p>
        <p class="center-text">Start at: {new Date(session.start_time).toLocaleTimeString()}</p>

        <button on:click={continueSession}>Continue Session</button>
        <button on:click={startNewSession}>Start New Session</button>
    {:else}
        <div>
            <MakeGains session={session} />
        </div>
    {/if}
{/if}