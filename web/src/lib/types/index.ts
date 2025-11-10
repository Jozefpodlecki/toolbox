export interface LoadResult {
    sessionId: string;
}

export interface Paged<T> {
    page: number;
    items: T[];
    total: number;
}

export interface Program {
    name: string;
    path: string;
}

export interface Process {
    id: number;
    parentId: number;
    name: string;
    exePath: string | null;
    sessionId: number | null;
    memoryKb: number | null;
    cpuTimeMs: number | null;
    startTimeFiletime: number | null;
    iconPath: string | null;
}

export interface ProcessNode {
    process: Process;
    children: ProcessNode[];
}

export * from "./updater";