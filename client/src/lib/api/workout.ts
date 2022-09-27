import type { StatusItem } from '../models/status-item';
import type { User } from '../models/user';
import type { Workout } from '../models/workout';
import { postReqeust, RequestPath, getAll, get, del } from "./shared";

export async function insertWorkout(user: User, row: Workout) {
    return await insertOrUpdateWorkout(RequestPath.InsertWorkout, user, row);
}

export async function updateWorkout(user: User, row: Workout) {
    return await insertOrUpdateWorkout(RequestPath.UpdateWorkout, user, row);
}

export async function getAllWorkouts(user: User) {
    return await getAll<Workout>(RequestPath.GetAllWorkouts, user);
}

export async function getWorkout(id: string, user: User) {
    return await get<Workout>(RequestPath.GetAllWorkouts, id, user);

}

export async function deleteWorkout(id: string, user: User) {
    return await del(RequestPath.DeleteWorkout, id, user);
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

async function insertOrUpdateWorkout(url: string, user: User, row: Workout): Promise<StatusItem<Workout>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }

    const { token } = user;
    const resp = await fetch(url, postReqeust(token, row));

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