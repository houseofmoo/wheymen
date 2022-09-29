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

<div class="page">
    <Title subtitle={"logout"} />
    <p>Logging out...</p>
</div>

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }

    p {
        text-align: center;
    }
</style>