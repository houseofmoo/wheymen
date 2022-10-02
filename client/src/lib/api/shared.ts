import type { User } from '../models/user';
import type { DbResponse } from '../models/db-response';
import { Loading } from "../stores/loading-store";

export enum RequestPath {
    InsertRoutine = `/api/routines/insert`,
    UpdateRoutine = `/api/routines/update`,
    GetAllRoutines = `/api/routines/get-all`,
    GetRoutine = `/api/routines/get`,
    DeleteRoutine = `api/routines/delete`,

    InsertWorkout = `/api/workouts/insert`,
    UpdateWorkout = `/api/workouts/update`,
    GetAllWorkouts = `/api/workouts/get-all`,
    GetWorkout = `/api/workouts/get`,
    GetUnrelatedWorkouts = `/api/workouts/get-all/unrelated`,
    DeleteWorkout = `/api/workouts/delete`,
}

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

export async function getAll<T>(url: RequestPath, user: User): Promise<DbResponse<T[]>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }
    
    Loading.start();

    const { token } = user;
    const resp = await fetch(url, postReqeust(token, ""));

    let response: DbResponse<T[]> = null;
    if (resp.status === 200) {
        const obj: T[] = await resp.json();
        response = {
            result: obj,
            count: obj.length,
            status: "success"
        }
    } else if (resp.status === 204) {
        response = {
            result: [],
            count: 0,
            status: "success"
        }
    } else {
        response = {
            result: null,
            count: -1,
            status: await resp.text()
        }
    }

    Loading.complete();
    return response;
}

export async function get<T>(url: RequestPath, id: string, user: User): Promise<DbResponse<T>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }
    
    Loading.start();

    const { token } = user;
    const completeUrl = `${url}/${id}`;
    const resp = await fetch(completeUrl, postReqeust(token, ""));

    let response: DbResponse<T> = null;
    if (resp.status === 200) {
        const obj: T = await resp.json();
        response = {
            result: obj,
            count: 1,
            status: "success"
        }
    } else if (resp.status === 204) {
        response = {
            result: null,
            count: 0,
            status: "success"
        }
    } else {
        response = {
            result: null,
            count: -1,
            status: await resp.text()
        }
    }

    Loading.complete();
    return response;
}

export async function del(url: RequestPath, id: string, user: User) {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }
    
    Loading.start();

    const { token } = user;
    const completeUrl = `${url}/${id}`;
    const resp = await fetch(completeUrl, postReqeust(token, ""));

    if (resp.status === 200 || resp.status === 204) {
        Loading.complete();
        return {
            status: "success"
        }
    } else {
        Loading.complete();
        return {
            status: await resp.text()
        }
    }
}