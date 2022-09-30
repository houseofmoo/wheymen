<script lang="ts">
    import {push} from 'svelte-spa-router'
    import Title from "../components/display/title.svelte";
</script>

<div class="page">
    <Title />
    <div class="text">
        <p>Keep a list of workout routines you perform regularly. Track your workout progress with stats about each workout and how much weight you lifted in the past.</p>
        <p>The worlds only bro operated work out tracking website. Probably. Honestly, I have no idea. But if you can't trust a bro, who can you trust?</p>
        <p>Oh, it's free too.</p>
        <button on:click={() => push('/signup')}>sign up</button>
        <button on:click={() => push('/login')}>login</button>
    </div>
</div>

<style>

    .text {
        display: grid;
        place-content: center;
        padding: 1rem;
        text-align: left;
    }
</style>