<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import { getSession} from "../api/session";
    import MakeGains from "../components/features/gains/make-gains.svelte";
    import type { Session } from "../models/session";
    import { Alert } from "../stores/alert-store";

    export let params = { id: null };
    let session_id = params.id;
    let session: Session = null;

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
            return;
        }

        const session_res = await getSession($UserStore, session_id);
        if (session_res.status_code === 200) {session_id
            session = session_res.result;
        } else if (session_res.status_code === 204) {
            Alert.setMsg("Session not found, returning to profile");
                push('/profile/routines');
                return;
        } else {
            Alert.setMsg("Error getting session, returning to proile");
            push('/profile/routines');
            return;
        }
    });

</script>

{#if $UserStore && session}
    <div>
        <MakeGains session={session} />
    </div>
{/if}