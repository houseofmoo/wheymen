import { createClient } from '@supabase/supabase-js'
import type { User } from '../models/user';

const SUPABASE_URL = "https://hqjjieowgxpwefoeixte.supabase.co";
const PUBLIC_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImhxamppZW93Z3hwd2Vmb2VpeHRlIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NjIxOTYxNTQsImV4cCI6MTk3Nzc3MjE1NH0.OYvSyodzN0s8NIQoOLZRfX2tq8utH9IPYUAK9mRr7nI";
const supabase = createClient(SUPABASE_URL, PUBLIC_KEY);

export async function getSessionFromLocalToken(): Promise<User> {
    try {
        // get locally stored token
        const storedToken = localStorage.getItem("supabase.auth.token");
        if (storedToken == null) {
            return null;
        }

        // parse to obj
        const tokenObj = JSON.parse(storedToken);
        if (!tokenObj.currentSession.access_token) {
            return null;
        }

        // validate token
        let resp = await supabase.auth.api.getUser(tokenObj.currentSession.access_token);
        if (resp.error === null) {
            return {
                email: tokenObj.currentSession.user.email,
                token: tokenObj.currentSession.access_token,
                id: tokenObj.currentSession.user.id,
                role: tokenObj.currentSession.user.role
            }
        }
        
        return null;
    } catch (e) {
        console.log("error while valdating locally stored token: ", e);
        return null;
    }
}

export async function doesUserExist(email: string) {
    try {
        const { data } = await supabase.rpc("does_user_exist", { user_email: email });
        return data;
    }
    catch (e) {
        console.log("error: ", e);
        return false;
    }
}

export async function signUp(email: string, password: string) {
    try {
        const { user, error } = await supabase.auth.signUp({
            email,
            password
        });

        return {
            email: user.email,
            id: user.id,
            error: error ? error.message : "success",
            status: error ? error.status : 200,
        };
    } catch (e) {
        return {
            email: "",
            id: "",
            error: e,
            status: 404, // what ever the server error code is
        };;
    }
}

export async function signIn(email: string, password: string): Promise<User> {
    try {
        const { user, session, error } = await supabase.auth.signIn({
            email,
            password
        });

        if (error) return null;
        if (!session) return null;

        return {
            email: user.email,
            token: session.access_token,
            id: user.id,
            role: user.role
        }
    } catch {
        return null;
    }
}

export async function signOut() {
    try {
        const { error } = await supabase.auth.signOut();
        return error === null;
    } catch {
        return false;
    }
}

export async function getSession() {
    try {
        const session = await supabase.auth.session();
        if (session) {
            return {
                email: session.user.email,
                token: session.access_token,
                id: session.user.id,
                role: session.user.role
            }
        }

        return null;
    } catch (e) {
        console.log("error getting user session: ", e)
        return null;
    }
}