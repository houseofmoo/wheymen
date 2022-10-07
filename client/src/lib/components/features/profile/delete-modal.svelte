<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { fade } from "svelte/transition";
    import { del } from "../../../api/shared";
    import { RequestTarget } from "../../../api/urls";
    import { UserStore } from "../../../stores/user-store";

    export let item: any = null;
    export let target: RequestTarget = RequestTarget.DeleteRoutine;

    const dispatch = createEventDispatcher();

    let name: string = "";
    let modal;

    export function show() {
        modal.showModal();
    }

    function close() {
        name = "";
        modal.close();
    }

    async function deleteItem() {
        // this can be a workout or a routine
        await del(target, item.id, $UserStore);
        close();
        dispatch('item-deleted');
    }

</script>

<dialog class="modal" bind:this={modal}>
    <div class="content">
        <p class="center-text">Type <span>{item.name}</span> in the box below to confirm</p>
        <input class="center-text styled-input" bind:value={name} placeholder="{item.name}" />
        <div class="action-buttons">
            <button on:click={close}>cancel</button>
            {#if name === item.name}
                <button on:click={deleteItem} transition:fade|local>delete</button>
            {/if}
        </div>
    </div>
</dialog>

<style>
    .modal {
        background-color: var(--primary-color-800);
        border: 1px solid black;
        max-width: 30em;
        width: 90%;
        margin-top: 20vh;
    }

    .content {
        display: grid;
        grid: 1fr / 1fr;
        place-content: center;
        place-items: center;
    }

    .modal::backdrop {
        background: black;
        opacity: 0.4;
    }

    p {
        color: var(--text-color);
    }

    .action-buttons {
        margin-top: 2em;
        display: grid;
        grid: auto / 1fr 1fr;
        grid-gap: 1em;
        place-items: center;
        place-content: center;
        width: 100%;
    }

    .action-buttons > button {
        width: 100%;
    }

    span {
        font-weight: 800;
    }
</style>