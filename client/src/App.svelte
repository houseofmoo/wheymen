<script lang="ts">
    import { onMount } from "svelte";
    import { push } from 'svelte-spa-router'
    import { getSessionFromLocalToken, getSession } from "./lib/api/auth";
    import { UserStore } from "./lib/stores/user-store";
    import Router from "svelte-spa-router";
    import AlertNotification from "./lib/components/display/alert-notification.svelte";
    import Loading from "./lib/components/display/loading.svelte";

    import AccountRecoveryPage from "./lib/pages/account-recovery-page.svelte";
    import CreateRoutinePage from "./lib/pages/create-routine-page.svelte";
    import CreateWorkoutPage from "./lib/pages/create-workout-page.svelte";
    import EditRoutinePage from "./lib/pages/edit-routine-page.svelte";
    import EditWorkoutPage from "./lib/pages/edit-workout-page.svelte";
    import LandingPage from "./lib/pages/landing-page.svelte";
    import LoginPage from "./lib/pages/login-page.svelte";
    import LogoutPage from "./lib/pages/logout-page.svelte";
    import ProfilePage from "./lib/pages/profile-page.svelte";
    import SignupPage from "./lib/pages/sign-up-page.svelte";
    import MakeGainsPage from "./lib/pages/make-gains-page.svelte";

    const routes = {
        '/': LandingPage,
        '/account-recovery': AccountRecoveryPage,
        '/create-routine': CreateRoutinePage,
        '/create-workout': CreateWorkoutPage,
        '/edit-routine/:id': EditRoutinePage,
        '/edit-workout/:id': EditWorkoutPage,
        '/login': LoginPage,
        '/logout': LogoutPage,
        '/make-gains/:id': MakeGainsPage,
        '/profile/:tab': ProfilePage,
        '/signup': SignupPage,
        '*': LandingPage
    }

    onMount(async () => {
        // attemp to login the user from their stored token
        $UserStore = await getSession();
        if ($UserStore == null) {
            $UserStore = await getSessionFromLocalToken();
        }
    });

 
</script>

<Loading />
<AlertNotification />
<main class="content">
    <Router {routes} />
</main>

<style>
    .content {
        margin: auto;
        padding-top: 1em;
        max-width: var(--max-width);
        min-width: var(--min-width);
    }
</style>
