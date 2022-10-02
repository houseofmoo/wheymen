import type { DbResponse } from '../models/db-response';
import type { User } from '../models/user';
import type { Workout, WorkoutRow, InsertWorkoutRow, UpsertWorkoutRow } from '../models/workout';
import { postReqeust, RequestPath, getAll, get, del } from "./shared";
import { Loading } from "../stores/loading-store";

export async function insertWorkout(user: User, workout: Workout, selected_routine_ids: string[], unselected_routine_ids: string[]) {
    Loading.start();
    const upsert: UpsertWorkoutRow<InsertWorkoutRow> = {
        workout_row: {
            user_id: workout.user_id,
            name: workout.name,
            category: workout.category,
            note: workout.note,
        },
        selected_routine_ids, 
        unselected_routine_ids 
    };
    const res = await insertOrUpdateWorkout(RequestPath.InsertWorkout, user, upsert);
    Loading.complete();
    return res;
}

export async function updateWorkout(user: User, workout: Workout, selected_routine_ids: string[], unselected_routine_ids: string[]) {
    Loading.start();
    const upsert: UpsertWorkoutRow<WorkoutRow> = {
        workout_row: {
            id: workout.id,
            user_id: workout.user_id,
            name: workout.name,
            category: workout.category,
            note: workout.note,
        },
        selected_routine_ids, 
        unselected_routine_ids 
    };
    const res = await insertOrUpdateWorkout(RequestPath.UpdateWorkout, user, upsert);
    Loading.complete();
    return res;
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

export async function getUnrelatedWorkouts(routine_id: string, user: User) {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }

    Loading.start();
    
    const { token } = user;
    const completeUrl = `${RequestPath.GetUnrelatedWorkouts}/${routine_id}`;
    const resp = await fetch(completeUrl, postReqeust(token, ""));

    let response: DbResponse<Workout[]> = null;
    if (resp.status === 200) {
        const obj: Workout[] = await resp.json()
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

async function insertOrUpdateWorkout<T>(url: string, user: User, upsert: T): Promise<DbResponse<Workout>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }

    const { token } = user;
    const resp = await fetch(url, postReqeust(token, upsert));

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