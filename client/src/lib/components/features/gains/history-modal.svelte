<script lang="ts">
    import type {  SessionWorkout } from "../../../models/session";
    import Modal from "../../display/modal.svelte";

    export let history: SessionWorkout[] = [];
    let showModal: () => void;
    let closeModal: () => void;

    export function showHistory() {
        showModal();
    }

</script>

<Modal bind:showModal={showModal} bind:closeModal={closeModal}>
    {#each history as moment}
        <p>{moment.workout_name}</p>
        {#each moment.sets as set}
            <p>{set.weight} x {set.reps}</p>
        {/each}
    {/each}
</Modal>

<style>
    p {
        color: var(--text-color);
    }
</style>