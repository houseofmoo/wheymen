export class DbResponse<T> {
    result: T;
    status_code: number;
    status_msg: string;
}