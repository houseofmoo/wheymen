<script lang="ts">
    import { slide } from "svelte/transition";
    import { Alert } from "../../stores/alert-store";
</script>

{#if $Alert}
    <div class="sticky" transition:slide|local>
        <div />
        <p>{$Alert}</p>
        <button class="close-button" on:click={() => $Alert = null}>x</button>
    </div>
{/if}

<style>
   .sticky {
        margin: auto;
		position: absolute;
		top: 0;
        left: 0;
        right: 0;
        padding: 0;
		background-color: var(--primary-color-400);
		color: var(--bg-color);
        border-top: none;
        border-left: none;
        border-right: none;
        border-bottom: solid 1px black;
		z-index: 1;
		text-align: center;
        box-sizing: border-box;
        width: 100%;
        max-width: var(--max-width);
        min-width: var(--min-width);
        display: grid;
        grid: 1fr / 1fr auto 1fr;

        place-content: center;
        place-items: center;
	}

    .sticky > p {
        margin: 1em;
    }

    .close-button {
        display: flex;
        margin:auto;
        border: none;
        color: var(--secondary-color);
        background-color: var(--primary-color-400);
        text-align: center;
        padding: 0;
        margin: 0;
    }
</style>