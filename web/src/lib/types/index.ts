export interface LoadResult {
    
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