<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { fade } from "svelte/transition";
    import KebabMenuIcon from "../display/icons/kebab-menu.svelte";

    let visible = false;
    let container;

    onMount(() => {
        document.addEventListener('click', hideOnClick);
    });

    onDestroy(() => {
        document.removeEventListener('click', hideOnClick);
    });

    function hideOnClick(e: Event) {
        if (!container.contains(e.target) && visible) {
            visible = false;
        } 
    }
</script>

<div class="container"  bind:this={container}>
    <button class="kebab-button" on:click={() => visible = !visible}><KebabMenuIcon /></button>
    {#if visible}
        <div class="menu large-text" transition:fade|local="{{duration: 100}}">
            <slot />
        </div>
    {/if}
</div>

<style>
    .container {
        position: relative;
        width: fit-content;
        margin-left: auto;
    }

    .kebab-button {
        border: none;
        background-color: transparent;
        padding: 0;
        margin: 0;
    }

    .menu {
        font-family: 'Space Grotesk', sans-serif;
        position: absolute;
        background-color: var(--primary-color-400);
        right: 1em;
        border: 1px solid black;
        padding: 1em;
        display: grid;
        grid: 1fr / 1fr;
        grid-gap: 0.5em;
        width: max-content;
        box-shadow: 3px 3px 10px black;
        z-index: 0;
    }
</style>