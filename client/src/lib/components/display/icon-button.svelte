<script lang="ts">
	export let type: string = 'button';
	export let icon;
	export let text = null;
</script>

<button on:click {type}>
    <div class="grid">
        <svelte:component this={icon} />
        {#if text}
            <div class="icon-text">{text}</div>
        {/if}
    </div>
</button>

<style>
    .grid {
        display: grid;
        grid: 1fr / 1fr;
        place-content: center;
        place-items: center;
        width: 100%;
    }
    
	button {
		background-color: var(--primary-color);
		border: none;
		cursor: pointer;
		font-weight: bold;
		padding: 0.5em;
	}

    button > div {
        text-align: center;
        color: var(--link-color);
    }

    .icon-text {
        font-size: var(--icon-font-size);
    }
</style>