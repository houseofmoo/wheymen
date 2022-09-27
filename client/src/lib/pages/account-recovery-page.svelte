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
            <input type="email" placeholder="email" bind:value={email} />
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
        margin: auto;
        padding: 0;
    }

    .form-info {
        display: grid;
        grid: 1fr / auto;
        margin: auto;
        padding: 1rem;
        text-align: left;
    }

    input {
        padding: 0.25rem;
    }

    form {
        display: grid;
        grid: 1fr / auto;
        margin: auto;
        width: 250px;
        padding: 0;
    }

    input {
        border: none;
        border-bottom: 1px solid var(--text-color);
        background: transparent;
        color: var(--text-color);
        font-weight: 900;
        font-size: var(--font-size);
        margin: 1rem;
    }

    input:focus {
        outline: none;
    }

    button {
        font-size: var(--font-size);
        background-color: var(--lightgrey);
        color: var(--text-color);
        margin: 0.5rem;
        padding: 0.75rem;
        border: solid 1px black;
        cursor: pointer;
        transition: border 0.25s;
    }

    button:hover {
        border: solid 1px white;
    }

    @media (max-width: 600px) {
        .form-sheet{
            grid: 1fr / auto;
        }
    }
</style>