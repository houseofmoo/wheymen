<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import { UserStore } from "../stores/user-store";
    import Title from "../components/display/title.svelte";
    import { signOut } from "../api/auth";

    onMount(async () => {
        if ($UserStore !== null) {
            await signOut();
        }
        push('/');
    });
</script>

<div>
    <Title subtitle={"logout"} />
    <p>Logging out...</p>
</div>

<style>
    p {
        text-align: center;
    }
</style>