<script lang="ts">
    import { Category } from "../../models/category";

    export let filter = "all";
</script>

<div class="category-selector">
    <select bind:value={filter}>
        <option value="all">show all</option>
        {#each Object.values(Category) as category}
            <option value={category}>{category}</option>
        {/each}
    </select>
</div>

<style>
    .category-selector {
        margin: 0;
        padding: 0;
    }

   select {
        padding: 0.25em;
   }
</style>