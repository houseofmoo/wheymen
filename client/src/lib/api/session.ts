import type { Session } from "../models/session";
import type { DbResponse } from "../models/db-response";
import type { User } from "../models/user";
import { postReqeust } from "./shared";
import { RequestTarget, generateUrl } from "./urls";
import { Loading } from "../stores/loading-store";

export async function startSession(user: User, routine_id: string): Promise<DbResponse<Session>> {
    if (user === null) {
        return {
            result: null,
            count: -1,
            status: "user is null",
        }
    }
    
    Loading.start();
    const { token } = user;
    const url = generateUrl(RequestTarget.StartSession, routine_id);
    const resp = await fetch(url, postReqeust(token, ""));
    
    let result: DbResponse<Session> = null;
    if (resp.status === 200) {
        const obj: Session = await resp.json()
        result = {
            result: obj,
            count: 1,
            status: "success"
        }
    } else if (resp.status === 204) {
        result =  {
            result: null,
            count: 0,
            status: "empty",
        }
    } else {
        result =  {
            result: null,
            count: -1,
            status: await resp.text()
        }
    }
    
    Loading.complete(); 
    return  result;
}