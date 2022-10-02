<script lang="ts">
    import { Loading } from "../../stores/loading-store";
    import Wheymen from "./icons/wheymen.svelte";
</script>

{#if $Loading}
    <div class="icon">
        <Wheymen scale={2} />
    </div>
    <div class="whole-area" />
{/if}


<style>
	.whole-area {
        position: absolute;
		height: 100vh;
		width: 100vw;
		top: 0;
		left: 0;
		opacity: 0.65;
		background-color: black;
		z-index: 98;
	}

	.icon {
        position: absolute;
        left: 0;
        right: 0;
        top: 0;
        bottom: 0;
        margin: auto;
        z-index: 99;
        height: fit-content;
        width: fit-content;
        animation: lift 1s infinite ease-in;
	}

	@keyframes lift {
        0% {
            top: 0;
            transform: rotate(0deg);
        }
        70% {
            top: -250px;
            transform: rotate(0deg);
        }
        100% {
            top: -0px;
            transform: rotate(0deg);
        }
	}
</style>