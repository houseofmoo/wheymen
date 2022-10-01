<script lang="ts">
    import { del, RequestPath } from "../../../api/shared";
    import { UserStore } from "../../../stores/user-store";

    export let item: any = null;
    export let url: RequestPath = RequestPath.DeleteRoutine;

    let name: string = "";
    let modal;

$: {
    console.log(name);
    console.log(item.name);
    console.log(name === item.name);
}

    export function show() {
        modal.showModal();
    }

    async function deleteItem() {
        await del(url, item.id, $UserStore);
        modal.close();
    }

</script>

<dialog class="modal" bind:this={modal}>
    <button class="close-button" on:click={() => modal.close()}>x</button>
    <div class="routine-selector">
        <p>Delete {item.name}?</p>
        <input bind:value={name} placeholder="Type name here to confirm deletion" />
        <button on:click={() => modal.close()}>cancel</button>
        {#if name === item.name}
            <button on:click={deleteItem}>delete</button>
        {/if}
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
        place-items: center;
        place-content: center;
        width: 100%;
        height: fit-content;
        overflow-y: scroll;
        scrollbar-width: none;
        z-index: 2;
    }

    .close-button {
        display: flex;
        margin-left: auto;
        margin-top: 0;
        padding-top: 0;
        border: none;
        color: var(--secondary-color);
        background-color: var(--primary-color-800);
        text-align: right;
    }

    p {
        color: var(--text-color);
    }
</style>