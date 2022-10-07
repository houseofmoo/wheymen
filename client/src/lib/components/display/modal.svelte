<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();
    let modal;

    export function showModal() {
        modal.showModal();
        dispatch('modal-show');
    }

    export function closeModal() {
        modal.close();
        dispatch('modal-close');
    }
</script>

<dialog class="modal" bind:this={modal}>
    <button class="close-button" on:click={() => modal.close()}>x</button>
    <div class="selector">
       <slot />
    </div>
</dialog>

<style>
    .modal {
        background-color: var(--primary-color-800);
        border: 1px solid black;
        max-width: 30em;
        width: 100%;
        margin-top: 10vh;
    }

    .modal::backdrop {
        background: black;
        opacity: 0.4;
    }

    .close-button {
        display: flex;
        margin-left: auto;
        margin-top: 0;
        padding-top: 0;
        border: none;
        color: var(--secondary-color);
        background-color: var(--primary-color-800);
    }

    .selector {
        scrollbar-width: none;
    }
</style>