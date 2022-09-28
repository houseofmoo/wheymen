<script lang="ts">
    import { Category } from "../../../models/category";

    export let selected = "arms";
</script>

<div class="category-selector">
    <select bind:value={selected}>
        {#each Object.values(Category) as category}
            <option value={category}>{category}</option>
        {/each}
    </select>
</div>

<style>
    .category-selector, select {
        width: 100%;
        margin: 0;
        padding: 0;
    }

   select {
        padding: 0.25em;
   }
</style>