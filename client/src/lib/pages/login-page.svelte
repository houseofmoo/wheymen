<script lang="ts">
    import { push, link } from 'svelte-spa-router'
    import { signIn } from '../api/auth';
    import { UserStore } from "../stores/user-store";
    import ErrorMessage from '../components/display/error-message.svelte';
    import Title from "../components/display/title.svelte";

    let email = "";
    let password = "";
    let statusMessage = null;

    async function onSubmit() {
        statusMessage = null;

        if (email.length <= 0) {
            statusMessage = "please enter you account email address";
            return;
        }

        if (password.length <= 0) {
            statusMessage = "please enter your password";
            return;
        }

        $UserStore = await signIn(email, password);
        if ($UserStore !== null) {
            email = "";
            password = "";
            push("/profile");
        } else {
            password = "";
            statusMessage = "email or password are incorrect";
        }
    }
</script>

<div>
    <Title subtitle={"login"} />
    <ErrorMessage errorMsg={statusMessage} />
    <div class="form-sheet">
        <form on:submit|preventDefault={onSubmit}>
            <input class="styled-input" type="email" placeholder="email" bind:value={email} />
            <input class="styled-input" type="password" placeholder="password" bind:value={password} />
            <button type="submit">login</button>
        </form>
        <div class="form-info">
            <p>Don't have an account yet? <a href="/signup" use:link>Sign up</a> for your free account now.</p>
            <p>Forgot your password? <a href="/account-recovery" use:link>Account recovery</a> to reset your password.</p>
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
        min-width: 15em;
        grid-gap: 2em;
    }

    .form-info {
        padding: 1rem;
        text-align: left;
    }
</style>