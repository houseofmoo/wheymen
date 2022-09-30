<script lang="ts">

    let visible = false;

</script>

<div class="container">
    <button class="kebab-button" on:click={() => visible = !visible}>...</button>

    <div class={visible ? "menu visible" : "menu hidden"}>
        <p>item 1</p>
        <p>item 1</p>
        <p>item 1</p>
        <p>item 1</p>
    </div>
</div>

<style>
    .container {
        position: relative;
        text-align: right;
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
        position: absolute;
        left: 10em;
        border: 1px solid black;
        background-color: var(--darkgrey);
        padding: 1em;
        min-width: 10em;
    }


    .hidden {
        display: none;
    
    }

    .visible {
        display: block;
    }
</style>