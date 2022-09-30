<script lang="ts">
    import { link } from 'svelte-spa-router'
    import ErrorMessage from '../components/display/error-message.svelte';
    import Title from "../components/display/title.svelte";

    let email = "";
    let password = "";
    let statusMessage = null;

    async function onSubmit() {
        statusMessage = null;

        // TODO: account recovery email 
    }
</script>

<div>
    <Title subtitle={"recovery"} />
    <ErrorMessage errorMsg={statusMessage} />
    <div class="form-sheet">
        <form on:submit|preventDefault={onSubmit}>
            <input class="account-input" type="email" placeholder="email" bind:value={email} />
            <button type="submit">send recovery email</button>
        </form>
        <div class="form-info">
            <p>Don't have an account yet? <a href="/signup" use:link>Sign up</a> for your free account now</p>
            <p>Already have an account? <a href="/login" use:link>Login</a> to your account</p>
        </div>
    </div>
    <div class="center-text">
        <a href="/" use:link>home</a>
    </div>
</div>

<style>
    .form-sheet {
        display: grid;
        grid: 1fr / auto;
        place-items: center;
        place-content: center;
    }

    form {
        display: grid;
        grid: 1fr / auto;
        max-width: 70%;
        min-width: 20em;
    }

    .form-info {
        padding: 1rem;
        text-align: left;
    }
</style>