import type { User } from '../models/user';
import type { StatusItem } from '../models/status-item'

export enum RequestPath {
    GetAllRoutines = `/api/routines/get-all`,
    GetAllWorkouts = `/api/workouts/get-all`,
    GetUnrelatedWorkouts = `/api/workouts/get-all/unrelated`,
    DeleteRoutine = `api/routines/delete`,

    InsertRoutine = `/api/routines/insert`,
    InsertWorkout = `/api/workouts/insert`,

    UpdateRoutine = `api/routines/update`,

    GetRoutine = `/api/routines/get`,
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

export async function getAll<T>(url: RequestPath, user: User): Promise<StatusItem<T[]>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }
    
    const { token } = user;
    const resp = await fetch(url, postReqeust(token, ""));
    if (resp.status === 200) {
        const obj: T[] = await resp.json();
        return {
            result: obj,
            count: obj.length,
            status: "success"
        }
    } else if (resp.status === 204) {
        return {
            result: [],
            count: 0,
            status: "success"
        }
    } else {
        return {
            result: null,
            count: -1,
            status: await resp.text()
        }
    }
}

export async function get<T>(url: RequestPath, id: string, user: User): Promise<StatusItem<T>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }
    
    const { token } = user;
    const completeUrl = `${url}/${id}`;
    const resp = await fetch(completeUrl, postReqeust(token, ""));

    if (resp.status === 200) {
        const obj: T = await resp.json();
        return {
            result: obj,
            count: 1,
            status: "success"
        }
    } else if (resp.status === 204) {
        return {
            result: null,
            count: 0,
            status: "success"
        }
    } else {
        return {
            result: null,
            count: -1,
            status: await resp.text()
        }
    }
}

export async function del<T>(url: RequestPath, id: string, user: User) {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }
    
    const { token } = user;
    const completeUrl = `${url}/${id}`;
    const resp = await fetch(completeUrl, postReqeust(token, ""));

    if (resp.status === 200 || resp.status === 204) {
        return {
            status: "success"
        }
    } else {
        return {
            status: await resp.text()
        }
    }
}