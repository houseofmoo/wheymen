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

<div class="page">
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
    <div class="nav">
        <a href="/" use:link>home</a>
    </div>
</div>

<style>
   .page {
        text-align: center;
        width: 100%;
        margin: 0;
        padding: 0;
    }

    .form-sheet {
        display: grid;
        grid: 1fr / 1fr 1fr;
        place-items: start center;
    }

    .form-info {
        padding: 1rem;
        text-align: left;
    }

    form {
        display: grid;
        grid: 1fr / auto;
        width: 100%;
    }

    @media (max-width: 600px) {
        .form-sheet {
            grid: 1fr / auto;
        }
    }
</style>