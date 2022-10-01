<script lang="ts">
    export let days: string[] = [];
</script>

<div class="days">
	<div class="days-upper">
		<label class="day-container">
			<input type="checkbox" value="sun" bind:group={days} />
			<div>sun</div>
		</label>
		<label class="day-container">
			<input type="checkbox" value="mon" bind:group={days} />
			<div>mon</div>
		</label>
		<label class="day-container">
			<input type="checkbox" value="tues" bind:group={days} />
			<div>tues</div>
		</label>
		<label class="day-container">
			<input type="checkbox" value="weds" bind:group={days} />
			<div>weds</div>
		</label>
	</div>
	<div class="days-lower">
		<label class="day-container">
			<input type="checkbox" value="thurs" bind:group={days} />
			<div>thurs</div>
		</label>
		<label class="day-container">
			<input type="checkbox" value="fri" bind:group={days} />
			<div>fri</div>
		</label>
		<label class="day-container">
			<input type="checkbox" value="sat" bind:group={days} />
			<div>sat</div>
		</label>
	</div>
</div>

<style>
    .day-container input {
		display: none;
	}

	.day-container input ~ div {
		background-color: var(--primary-color-400);
		border: solid 1px black;
		text-align: center;
        cursor: pointer;
        padding: 0.25em;
  		transition: border 0.25s;
	}

	.day-container input ~ div:hover {
		border: solid 1px var(--secondary-color);
	}

	.day-container input:checked ~ div {
		background-color: var(--secondary-color);
		border: solid 1px black;
		text-align: center;
	}

	.days {
		display: grid;
		grid: auto auto / auto;
        width: 100%;
        margin: 1em 0;
	}

	.days-upper {
		display: grid;
		grid: auto / repeat(4, 1fr);
		grid-gap: 0.25rem;
		text-align: center;
		margin: 0.25rem;
	}

	.days-lower {
		display: grid;
		grid: auto / repeat(3, 1fr);
		grid-gap: 0.25rem;
		text-align: center;
		margin: 0.25rem;
	}
</style>