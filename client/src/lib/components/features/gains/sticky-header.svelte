<script lang="ts">

    export let show = true;

</script>

{#if show}
    <div class="sticky">
        <slot />
    </div>
{/if}

<style>
    .sticky {
        position: sticky;
        top: 0;
        z-index: 1;
        background-color: var(--primary-color-400);
        border: 1px solid black;
    }
</style>