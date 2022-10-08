<script lang="ts">
    import { restElapsed, restTime, sessionElapsed } from "../../../stores/session-time";

    export let show_timer = true;

    function convertTo(time: number) {
        let totalSeconds = Math.round(time / 1000);
		let sec = totalSeconds % 60;
		let min = Math.floor(totalSeconds / 60);
		if (sec < 0) sec = 0;
		if (min < 0) min = 0;
		return (min < 10 ? '0' + min : min) + ':' + (sec < 10 ? '0' + sec : sec);
    }

    function resetRestTimer() {
        restTime.reset()
    }

</script>

{#if show_timer}
    <div class="content">
        <p>{convertTo($sessionElapsed)}</p>
        <p>{convertTo($restElapsed)}</p>
    </div>
{/if}

<style>
    .content {
        position: sticky;
        top: 0;
        z-index: 0;
        display: grid;
        grid: 1fr / 1fr 1fr;
        place-content: center;
        place-items: center;
        background-color: var(--primary-color-400);
        border: 1px solid black;
    }
</style>