import type { Routine, RoutineRow, InsertRoutineRow } from "../models/routine";
import type { DbResponse } from "../models/db-response";
import type { User } from "../models/user";
import { postReqeust, getAll, get, del } from "./shared";
import { RequestTarget, generateUrl } from "./request-target";
import { Loading } from "../stores/loading-store";

export async function insertRoutine(user: User, routine: Routine): Promise<DbResponse<Routine>> {
    const routine_row: InsertRoutineRow = {
        user_id: routine.user_id,
        name: routine.name,
        days: routine.days,
        last_completed: routine.last_completed,
        note: routine.note,
        workouts: routine.workouts.map(x => x.id),
    }
    return await insertOrUpdateRoutine(RequestTarget.InsertRoutine, user, routine_row);
}

export async function updateRoutine(user: User, routine: Routine): Promise<DbResponse<Routine>> {
    const routine_row: RoutineRow = {
        id: routine.id,
        user_id: routine.user_id,
        name: routine.name,
        days: routine.days,
        last_completed: routine.last_completed,
        note: routine.note,
        workouts: routine.workouts.map(x => x.id),
    }
    return await insertOrUpdateRoutine(RequestTarget.UpdateRoutine, user, routine_row);
}

export async function getAllRoutines(user: User): Promise<DbResponse<Routine[]>> {
    return await getAll<Routine>(RequestTarget.GetAllRoutines, user);
}

export async function getRoutine(routine_id: string, user: User): Promise<DbResponse<Routine>> {
    return await get<Routine>(RequestTarget.GetRoutine, routine_id, user);
}

export async function deleteRoutine(routine_id: string, user: User): Promise<DbResponse<Routine>> {
    return await del(RequestTarget.DeleteRoutine, routine_id, user);
}

async function insertOrUpdateRoutine<T>(target: RequestTarget, user: User, routine_row: T): Promise<DbResponse<Routine>> {
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
        const res = await fetch(url, postReqeust(token, routine_row));
    
        if (res.status === 200) {
            return {
                result: await res.json() as Routine,
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