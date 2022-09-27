import type { StatusItem } from '../models/status-item';
import type { User } from '../models/user';
import type { Workout } from '../models/workout';
import { postReqeust, RequestPath } from "./shared";

export async function insert(url: RequestPath, user: User, row: Workout): Promise<StatusItem<Workout>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }

    const { token } = user;
    const resp = await fetch(RequestPath.InsertWorkout, postReqeust(token, row));

    if (resp.status === 200) {
        const obj: Workout = await resp.json()
        return {
            result: obj,
            count: 1,
            status: "success"
        }
    } else if (resp.status === 204) {
        return {
            result: null,
            count: 0,
            status: "empty",
        }
    } else {
        return {
            result: null,
            count: -1,
            status: await resp.text()
        }
    }
}

export async function getUnrelatedWorkouts(id: string, user: User) {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }
    
    const { token } = user;
    const completeUrl = `${RequestPath.GetUnrelatedWorkouts}/${id}`;
    const resp = await fetch(completeUrl, postReqeust(token, ""));

    if (resp.status === 200) {
        const obj: Workout[] = await resp.json()
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