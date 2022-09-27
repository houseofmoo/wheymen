import type { Routine, RoutineRow } from "../models/routine";
import type { StatusItem } from "../models/status-item";
import type { User } from "../models/user";
import { postReqeust, RequestPath, getAll, get, del } from "./shared";

export async function insertRoutine(user: User, row: RoutineRow, workoutIds: string[]) {
    return await insertOrUpdateRoutine(RequestPath.InsertRoutine, user, row, workoutIds);
}

export async function updateRoutine(user: User, row: RoutineRow, workoutIds: string[]) {
    return await insertOrUpdateRoutine(RequestPath.UpdateRoutine, user, row, workoutIds);
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

async function insertOrUpdateRoutine(url: string, user: User, row: RoutineRow, workoutIds: string[]): Promise<StatusItem<Routine>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }

    const { token } = user;
    const resp = await fetch(url, postReqeust(token, { routine: row, workout_ids: workoutIds }));

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