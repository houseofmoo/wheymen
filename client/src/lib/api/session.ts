import type { Session } from "../models/session";
import type { DbResponse } from "../models/db-response";
import type { User } from "../models/user";
import { postReqeust, del } from "./shared";
import { RequestTarget, generateUrl } from "./request-target";
import { Loading } from "../stores/loading-store";

export async function startSession(user: User, routine_id: string): Promise<DbResponse<Session>> {
    if (!user) {
        return {
            result: null,
            status_code: 401,
            status_msg: "user is not logged in",
        }
    }
    
    try {
        Loading.start();

        const { token } = user;
        const url = generateUrl(RequestTarget.StartSession, routine_id);
        const res = await fetch(url, postReqeust(token, ""));
        
        if (res.status === 200) {
            return {
                result: await res.json() as Session,
                status_code: res.status,
                status_msg: "success"
            }
        } else if (res.status === 204) {
            return {
                result: null,
                status_code: res.status,
                status_msg: "empty",
            }
        } else {
            return {
                result: null,
                status_code: res.status,
                status_msg: await res.text(),
            }
        }
    } catch (e) {
        return {
            result: null,
            status_msg: e.toString(),
            status_code: 400,
        }
    } finally {
        Loading.complete();
    }
}

export async function continueSession(user: User, session_id: string): Promise<DbResponse<Session>> {
    if (!user) {
        return {
            result: null,
            status_code: 401,
            status_msg: "user is not logged in",
        }
    }
    
    try {
        Loading.start();

        const { token } = user;
        const url = generateUrl(RequestTarget.ContinueSession, session_id);
        const res = await fetch(url, postReqeust(token, ""));
        
        if (res.status === 200) {
            return {
                result: await res.json() as Session,
                status_code: res.status,
                status_msg: "success"
            }
        } else if (res.status === 204) {
            return {
                result: null,
                status_code: res.status,
                status_msg: "empty",
            }
        } else {
            return {
                result: null,
                status_code: res.status,
                status_msg: await res.text(),
            }
        }
    } catch (e) {
        return {
            result: null,
            status_msg: e.toString(),
            status_code: 400,
        }
    } finally {
        Loading.complete();
    }
}

export async function updateSession(user: User, session: Session): Promise<DbResponse<Session>> {
    if (!user) {
        return {
            result: null,
            status_code: 401,
            status_msg: "user is not logged in",
        }
    }
    
    try {
        Loading.start();

        const { token } = user;
        const url = generateUrl(RequestTarget.updateSession);
        const res = await fetch(url, postReqeust(token, session));
        
        if (res.status === 200) {
            return {
                result: await res.json() as Session,
                status_code: res.status,
                status_msg: "success"
            }
        } else if (res.status === 204) {
            return {
                result: null,
                status_code: res.status,
                status_msg: "empty",
            }
        } else {
            return {
                result: null,
                status_code: res.status,
                status_msg: await res.text(),
            }
        }
    } catch (e) {
        return {
            result: null,
            status_msg: e.toString(),
            status_code: 400,
        }
    } finally {
        Loading.complete();
    }
}

export async function deleteSession(user: User, session_id: string) {
    return await del(RequestTarget.DeleteSession, session_id, user);
}