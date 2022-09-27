<script lang="ts">
    import { UserStore } from "../../stores/user-store";
    import { signIn } from '../../api/auth';

    let email = "";
    let password = "";
    
    async function login() {
        $UserStore = await signIn(email, password);
        // emit login complete
    }
</script>

<div class="content">
    <input type="email" placeholder="email" bind:value={email} />
    <input type="password" placeholder="password" bind:value={password} />
    <button on:click={login}>Login</button>
</div>

<style>
    .content {
        display: grid;
        grid: 1fr / auto;
    }
</style>