<script lang="ts">
    import { onMount } from "svelte";
    import { push } from 'svelte-spa-router'
    import { getSessionFromLocalToken, getSession } from "./lib/api/auth";
    import { UserStore } from "./lib/stores/user-store";
    import Router from "svelte-spa-router";
    import AccountRecoveryPage from "./lib/pages/account-recovery-page.svelte";
    import CreateRoutinePage from "./lib/pages/create-routine-page.svelte";
    import CreateWorkoutPage from "./lib/pages/create-workout-page.svelte";
    import EditRoutine from "./lib/pages/edit-routine.svelte";
    import EditWorkout from "./lib/pages/edit-workout.svelte";
    import LandingPage from "./lib/pages/landing-page.svelte";
    import LoginPage from "./lib/pages/login-page.svelte";
    import LogoutPage from "./lib/pages/logout-page.svelte";
    import ProfilePage from "./lib/pages/profile-page.svelte";
    import SignupPage from "./lib/pages/sign-up-page.svelte";

    const routes = {
        '/': LandingPage,
        '/account-recovery': AccountRecoveryPage,
        '/create-routine': CreateRoutinePage,
        '/create-workout': CreateWorkoutPage,
        '/edit-routine/:id': EditRoutine,
        '/edit-workout/:id': EditWorkout,
        '/login': LoginPage,
        '/logout': LogoutPage,
        '/profile': ProfilePage,
        '/signup': SignupPage,
        '*': LandingPage
    }

    onMount(async () => {
        $UserStore = await getSession();
        if ($UserStore == null) {
            $UserStore = await getSessionFromLocalToken();
        }

        if ($UserStore !== null) {
            push('/profile');
        }
    });

 
</script>

<main class="content">
    <Router {routes} />
</main>

<style>
    .content {
        display: flex;
        margin: auto;
        padding: 1em;
        max-width: 30em;
        min-width: 19em;
    }
</style>
