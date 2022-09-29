<script lang="ts">
    import { link } from 'svelte-spa-router'
    import { signUp, doesUserExist } from '../api/auth';
    import ErrorMessage from '../components/display/error-message.svelte';
    import Title from "../components/display/title.svelte";

    let email = "";
    let password = "";
    let verifyPassword = "";
    let pageState: "signup" | "complete" | "exists" = "signup";
    let statusMessage = null;

    $: {
        if (verifyPassword && verifyPassword.length > 0) {
            if (verifyPassword.length < 6) {
                statusMessage = "Passwords do not match";
            } else if (password !== verifyPassword) {
                statusMessage = "Passwords do not match";
            }
            else {
                statusMessage = null;
            }
        } else {
            statusMessage = null;
        }
    }

    async function onSubmit() {
        statusMessage = null;

        if (email.length <= 0) {
            statusMessage = "Please enter a valud email";
            return;
        }

        if (password.length < 6) {
            statusMessage = "Password must be at least 6 characters long";
            return;
        }

        if (password !== verifyPassword) {
            statusMessage = "Passwords do not match"
        }

        // is user already signed up
        const userExists = await doesUserExist(email);
        if (userExists) {
            statusMessage = `User with ${email} already exists`;
            pageState = "exists";
            email = "";
            password = "";
            verifyPassword = "";
            return;
        }

        // send sign up request to auth
        const signUpResp = await signUp(email, password);
        if (signUpResp.status !== 200) {
            statusMessage = "Error while signing up. Please try again";
            pageState = "signup";
            email = "";
            password = "";
            verifyPassword = "";
            return;
        }

        email = "";
        password = "";
        verifyPassword = "";
        pageState = "complete";
    }
</script>

<div class="page">
    <Title subtitle={"sign up"} />
    {#if pageState === "signup"}
        <ErrorMessage errorMsg={statusMessage} />
        <div class="form-sheet">
            <form on:submit|preventDefault={onSubmit}>
                <input class="account-input" type="email" placeholder="email" bind:value={email} />
                <input class="account-input" type="password" placeholder="password" bind:value={password} />
                <input class="account-input" type="password" placeholder="verify password" bind:value={verifyPassword} />
                <button type="submit">sign up</button>
            </form>
            <div class="form-info">
                <p>Already have an account? <a href="/login" use:link>Login</a></p>
                <p>Forgot your password? <a href="/account-recovery" use:link>Account recovery</a></p>
                <p>We don't actually want your email, we just need it to identify your account. The only emails you'll get from <a href="/" use:link>wheymen.net</a> are an email address confirmation and password recovery. Cause you know, passwords are hard.</p>
                <p>Please don't use your banking password. It's just a workout tracker, not financial information bro.</p>
            </div>
        </div>
    {:else if pageState === "complete"}
        <div>
            <p>Check out your email at {email} to complete sign up!</p>
            <p>This page can be closed</p>
        </div>
    {:else if pageState === "exists"}
        <div>
            <p>A user with email {email} exists. You can <a href="/login" use:link>login</a> or <a href="/account-recovery" use:link>recover your password</a></p>
        </div>
    {/if}
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