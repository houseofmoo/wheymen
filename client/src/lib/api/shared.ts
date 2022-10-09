import type { User } from '../models/user';
import type { DbResponse } from '../models/db-response';
import { Loading } from "../stores/loading-store";
import { RequestTarget, generateUrl } from "./request-target";


export function postReqeust(jwt: string, content: any) {
    return {
        method: 'POST',
        body: JSON.stringify(content),
        headers: {
            'Content-Type': 'application/json',
            'Authorization': jwt,
        }
    };
}

export async function getAll<T>(target: RequestTarget, user: User): Promise<DbResponse<T[]>> {
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
        const url = generateUrl(target);
        const res = await fetch(url, postReqeust(token, ""));
        
        if (res.status === 200) {
            return {
                result: await res.json() as T[],
                status_code: res.status,
                status_msg: "success"
            }
        } else if (res.status === 204) {
            return {
                result: [],
                status_code: res.status,
                status_msg: "empty"
            }
        } else {
            return {
                result: null,
                status_code: res.status,
                status_msg: await res.text()
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

export async function get<T>(target: RequestTarget, id: string, user: User): Promise<DbResponse<T>> {
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
        const url = generateUrl(target, id);
        const res = await fetch(url, postReqeust(token, ""));
    
        if (res.status === 200) {
            return {
                result: await res.json() as T,
                status_code: res.status,
                status_msg: "success"
            }
        } else if (res.status === 204) {
            return {
                result: null,
                status_code: res.status,
                status_msg: "empty"
            }
        } else {
            return {
                result: null,
                status_code: res.status,
                status_msg: await res.text()
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

export async function del(target: RequestTarget, id: string, user: User) {
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
        const url = generateUrl(target, id);
        const res = await fetch(url, postReqeust(token, ""));

        if (res.status === 200 || res.status === 204) {
            return {
                result: null,
                status_code: res.status,
                status_msg: "success"
            }
        } else {
            return {
                result: null,
                status_code: res.status,
                status_msg: await res.text()
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