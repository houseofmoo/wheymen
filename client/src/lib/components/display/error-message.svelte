<script lang="ts">
    export let errorMsg: string = null;
</script>

<p class="{errorMsg ? "visible" : "hidden"}">{errorMsg}</p>

<style>
    p {
        margin: 0;
        padding: 1em;
        text-align: center;
        color: red;
    }

    .visible {
        opacity: 1;
        transition: all 1s;
    }

    .hidden {
        opacity: 0;
        transition: all 1s;
    }
</style>