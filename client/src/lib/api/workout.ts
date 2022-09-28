import type { StatusItem } from '../models/status-item';
import type { User } from '../models/user';
import type { Workout, WorkoutRow, InsertWorkoutRow } from '../models/workout';
import { postReqeust, RequestPath, getAll, get, del } from "./shared";

export async function insertWorkout(user: User, workout: Workout) {
    const workout_row: InsertWorkoutRow =  {
        user_id: workout.user_id,
        name: workout.name,
        category: workout.category,
        note: workout.note,
    }
    return await insertOrUpdateWorkout(RequestPath.InsertWorkout, user, workout_row);
}

export async function updateWorkout(user: User, workout: Workout) {
    const workout_row: WorkoutRow =  {
        id: workout.id,
        user_id: workout.user_id,
        name: workout.name,
        category: workout.category,
        note: workout.note,
    }
    return await insertOrUpdateWorkout(RequestPath.UpdateWorkout, user, workout_row);
}

export async function getAllWorkouts(user: User) {
    return await getAll<Workout>(RequestPath.GetAllWorkouts, user);
}

export async function getWorkout(id: string, user: User) {
    return await get<Workout>(RequestPath.GetWorkout, id, user);

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

async function insertOrUpdateWorkout<T>(url: string, user: User, workout_row: T): Promise<StatusItem<Workout>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }

    const { token } = user;
    const resp = await fetch(url, postReqeust(token, workout_row));

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