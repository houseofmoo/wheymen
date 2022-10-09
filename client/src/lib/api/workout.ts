import type { DbResponse } from '../models/db-response';
import type { User } from '../models/user';
import type { Workout, WorkoutRow, InsertWorkoutRow, UpsertWorkoutRow } from '../models/workout';
import { postReqeust, getAll, get, del } from "./shared";
import { Loading } from "../stores/loading-store";
import { RequestTarget, generateUrl } from "./request-target";

export async function insertWorkout(user: User, workout: Workout, selected_routine_ids: string[], unselected_routine_ids: string[]): Promise<DbResponse<Workout>> {
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
    return await insertOrUpdateWorkout(RequestTarget.InsertWorkout, user, upsert);
}

export async function updateWorkout(user: User, workout: Workout, selected_routine_ids: string[], unselected_routine_ids: string[]): Promise<DbResponse<Workout>> {
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
    return await insertOrUpdateWorkout(RequestTarget.UpdateWorkout, user, upsert);
}

export async function getAllWorkouts(user: User): Promise<DbResponse<Workout[]>> {
    return await getAll<Workout>(RequestTarget.GetAllWorkouts, user);
}

export async function getWorkout(id: string, user: User): Promise<DbResponse<Workout>> {
    return await get<Workout>(RequestTarget.GetWorkout, id, user);

}

export async function deleteWorkout(id: string, user: User): Promise<DbResponse<Workout>> {
    return await del(RequestTarget.DeleteWorkout, id, user);
}

export async function getUnrelatedWorkouts(routine_id: string, user: User): Promise<DbResponse<Workout>> {
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
        const url = generateUrl(RequestTarget.GetUnrelatedWorkouts, [ routine_id ]);
        const res = await fetch(url, postReqeust(token, ""));

        let response: DbResponse<Workout[]> = null;
        if (res.status === 200) {
            response = {
                result: await res.json() as Workout[],
                status_code: res.status,
                status_msg: "success"
            }
        } else if (res.status === 204) {
            response = {
                result: [],
                status_code: res.status,
                status_msg: "empty",
            }
        } else {
            response = {
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

async function insertOrUpdateWorkout<T>(target: RequestTarget, user: User, upsert: T): Promise<DbResponse<Workout>> {
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
        const res = await fetch(url, postReqeust(token, upsert));

        if (res.status === 200) {
            return {
                result: await res.json() as Workout,
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