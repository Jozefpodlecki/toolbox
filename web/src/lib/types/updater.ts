export type UpdateStatus = {
    type: "idle";
} | {
    type: "checking";
} | {
    type: "failed";
    value: string;
} | {
    type: "newVersion";
    value: string;
} | {
    type: "downloading";
    value: {
        version: string;
        total: number;
        length: number;
    }
} | {
    type: "downloaded";
    value: string;
} | {
    type: "latest";
    version: string;
}