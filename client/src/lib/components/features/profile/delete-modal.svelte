<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { fade } from "svelte/transition";
    import { del, RequestPath } from "../../../api/shared";
    import { UserStore } from "../../../stores/user-store";

    export let item: any = null;
    export let url: RequestPath = RequestPath.DeleteRoutine;

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
        await del(url, item.id, $UserStore);
        dispatch('item-deleted');
        modal.close();
    }

</script>

<dialog class="modal" bind:this={modal}>
    <div class="routine-selector">
        <p>Type <span>{item.name}</span> to confirm delete request</p>
        <input class="styled-input" bind:value={name} placeholder="{item.name}" />
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
        width: 60vw;
        max-width: 60em;
    }

    .modal::backdrop {
        background: black;
        opacity: 0.4;
    }

    .routine-selector {
        display: grid;
        grid: 1fr / 1fr;
        grid-gap: 1em;
        place-items: center;
        place-content: center;
        width: 100%;
        height: fit-content;
        overflow-y: scroll;
        scrollbar-width: none;
        z-index: 2;
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