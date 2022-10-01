<script lang="ts">
    import { link } from 'svelte-spa-router'
    import { signUp, doesUserExist } from '../api/auth';
    import ErrorMessage from '../components/display/error-message.svelte';
    import Title from "../components/display/title.svelte";

    let email = "";
    let password = "";
    let verify_pass = "";
    let page_state: "signup" | "complete" | "exists" = "signup";
    let status_msg = null;

    $: {
        if (verify_pass && verify_pass.length > 0) {
            if (verify_pass.length < 6) {
                status_msg = "Passwords do not match";
            } else if (password !== verify_pass) {
                status_msg = "Passwords do not match";
            }
            else {
                status_msg = null;
            }
        } else {
            status_msg = null;
        }
    }

    async function onSubmit() {
        status_msg = null;

        if (email.length <= 0) {
            status_msg = "Please enter your email";
            return;
        }

        if (password.length < 6) {
            status_msg = "Password must be at least 6 characters long";
            return;
        }

        if (password !== verify_pass) {
            status_msg = "Passwords do not match"
        }

        // is user already signed up
        const userExists = await doesUserExist(email);
        if (userExists) {
            status_msg = `User with ${email} already exists`;
            page_state = "exists";
            email = "";
            password = "";
            verify_pass = "";
            return;
        }

        // send sign up request to auth
        const signUpResp = await signUp(email, password);
        if (signUpResp.status !== 200) {
            status_msg = "Error while signing up. Please try again";
            page_state = "signup";
            email = "";
            password = "";
            verify_pass = "";
            return;
        }

        email = "";
        password = "";
        verify_pass = "";
        page_state = "complete";
    }
</script>

<div>
    <Title subtitle={"sign up"} />
    {#if page_state === "signup"}
        <ErrorMessage errorMsg={status_msg} />
        <div class="form-sheet">
            <form on:submit|preventDefault={onSubmit}>
                <input class="styled-input" type="email" placeholder="email" bind:value={email} />
                <input class="styled-input" type="password" placeholder="password" bind:value={password} />
                <input class="styled-input" type="password" placeholder="verify password" bind:value={verify_pass} />
                <button type="submit">sign up</button>
            </form>
            <div class="form-info">
                <p>Already have an account? <a href="/login" use:link>Login</a>. Forgot your password? <a href="/account-recovery" use:link>Account recovery</a></p>
                <p>We don't actually want your email, we just need it to identify your account. The only emails you'll get from <a href="/" use:link>wheymen.net</a> are an email address confirmation and password recovery. Cause you know, passwords are hard.</p>
                <p>Please don't use your banking password. It's just a workout tracker, not financial information bro.</p>
            </div>
        </div>
        <div class="center-text">
            <a href="/" use:link>home</a>
        </div>
    {:else if page_state === "complete"}
        <div class="center-text">
            <p>Check out your email at {email} to complete sign up</p>
        </div>
    {:else if page_state === "exists"}
        <div class="center-text">
            <p>A user with email {email} exists. You can <a href="/login" use:link>login</a> or <a href="/account-recovery" use:link>recover your password</a>.</p>
        </div>
    {/if}
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