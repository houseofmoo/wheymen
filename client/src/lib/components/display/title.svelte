<script lang="ts">
    export let title = "Wheymen"
    export let subtitle: string = "net";
</script>

<div class="content">
    <h1>{title}</h1>
    {#if subtitle}
    <div class="subtitle">
        <hr />
        <h3>{subtitle}</h3>
    </div>
    {/if}
</div>

<style>
    .content {
        display: grid;
        grid: 1fr / auto;
        margin: auto;
        padding: 0;
        width: fit-content;
        margin-bottom: 1em;
    }

    h1 {
        color: white;
        margin: 0;
        padding: 0;
        text-align: center;
    }
    
    .subtitle {
        display: grid;
        grid: auto / 1fr auto;
        margin: auto;
        width: 100%;
        grid-gap: 0.3em;
        margin-top: -10px;
    }

    hr {
        color: var(--orange);
        background-color: var(--orange);
        width: 100%;
        margin-top: 10px;
        height: 1px;
    }

    h3 {
        background-color: transparent;
        margin: 0;
        margin-left: auto;
        color: var(--orange);
    }
</style>