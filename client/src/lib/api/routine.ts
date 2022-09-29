import type { Routine, RoutineRow, InsertRoutineRow } from "../models/routine";
import type { DbResponse } from "../models/db-response";
import type { User } from "../models/user";
import { postReqeust, RequestPath, getAll, get, del } from "./shared";

export async function insertRoutine(user: User, routine: Routine) {
    const routine_row: InsertRoutineRow = {
        user_id: routine.user_id,
        name: routine.name,
        days: routine.days,
        last_completed: routine.last_completed,
        note: routine.note,
        workouts: routine.workouts.map(x => x.id),
    }
    return await insertOrUpdateRoutine(RequestPath.InsertRoutine, user, routine_row);
}

export async function updateRoutine(user: User, routine: Routine) {
    const routine_row: RoutineRow = {
        id: routine.id,
        user_id: routine.user_id,
        name: routine.name,
        days: routine.days,
        last_completed: routine.last_completed,
        note: routine.note,
        workouts: routine.workouts.map(x => x.id),
    }
    return await insertOrUpdateRoutine(RequestPath.UpdateRoutine, user, routine_row);
}

export async function getAllRoutines(user: User) {
    return await getAll<Routine>(RequestPath.GetAllRoutines, user);
}

export async function getRoutine(id: string, user: User) {
    return await get<Routine>(RequestPath.GetRoutine, id, user);
}

export async function deleteRoutine(id: string, user: User) {
    return await del(RequestPath.DeleteRoutine, id, user);
}

async function insertOrUpdateRoutine<T>(url: string, user: User, routine_row: T): Promise<DbResponse<Routine>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }

    const { token } = user;
    const resp = await fetch(url, postReqeust(token, routine_row));

    if (resp.status === 200) {
        const obj: Routine = await resp.json()
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