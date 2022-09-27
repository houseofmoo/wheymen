<script lang="ts">
    import { onMount } from "svelte";
    import { push } from "svelte-spa-router";
    import Title from "../components/display/title.svelte";
    import { UserStore } from "../stores/user-store";

    onMount(async () => {
        // if user landed here without being logged in, send them away
        if ($UserStore === null) {
            push('/login');
            return;
        }
    });
</script>

{#if $UserStore}
    <div class="page">
        <Title subtitle={"account"} />
        <div>
            <p>Hello {$UserStore.email}</p>
            <button on:click={() => push('/logout')}>logout</button>
        </div>
    </div>
{/if}

<style>
    .page {
        width: 100%;
        margin: 0;
        padding: 0;
    }
</style>