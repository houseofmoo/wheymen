<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { fade } from "svelte/transition";
    import { del } from "../../../api/shared";
    import { RequestTarget } from "../../../api/urls";
    import { UserStore } from "../../../stores/user-store";
    import Modal from "../../display/modal.svelte";

    export let item: any = null;
    export let target: RequestTarget = RequestTarget.DeleteRoutine;

    const dispatch = createEventDispatcher();

    let name: string = "";
    let showModal: () => void;
    let closeModal: () => void;

    export function show() {
        showModal();
    }

    function cancel() {
        cleanUpName()
        closeModal();
    }

    function cleanUpName() {
        name = "";
    }

    async function deleteItem() {
        // this can be a workout or a routine
        await del(target, item.id, $UserStore);
        close();
        dispatch('item-deleted');
    }

</script>

<Modal bind:showModal={showModal} bind:closeModal={closeModal} on:modal-close={cleanUpName}>
    <div class="content">
        <p class="center-text">Type <span>{item.name}</span> in the box below to confirm</p>
        <input class="center-text styled-input" bind:value={name} placeholder="{item.name}" />
        <div class="action-buttons">
            <button on:click={cancel}>cancel</button>
            {#if name === item.name}
                <button on:click={deleteItem} transition:fade|local>delete</button>
            {/if}
        </div>
    </div>
</Modal>
    

<style>
    .content {
        display: grid;
        grid: 1fr / 1fr;
        place-content: center;
        place-items: center;
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