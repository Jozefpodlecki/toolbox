
export interface DiskPartition {
    name: string;
    fsType: string;
    total: number;
    totalFormatted: string;
    used: string;
    usedFormatted: string;
    free: string;
    freeFormatted: string;
    usagePercent?: number;
};

export type DiskInfo = {
    model?: string;
    diskType?: string;
    partitions: DiskPartition[];
};

export interface MemoryInfo {
    totalPhys: number;
    totalPhysFormatted: string;
    availPhys: number;
    availPhysFormatted: string;
    totalPagefile: number;
    totalPagefileFormatted: string;
    availPagefile: number;
    availPagefileFormatted: string;
    totalVirtual: number;
    totalVirtualFormatted: string;
    availVirtual: number;
    availVirtualFormatted: string;
    memoryLoad: number; 
}

export interface DashboardStats {
    programsCount: number;
    activeProcesses: number;
    memory: MemoryInfo;
    disks: DiskInfo[];
}

export interface GetProcessArgs {
    name: string | null;
    display: "list" | "hierarchy";
    page: number;
    pageSize: number;
}

export interface GetProgramsArgs {
    page: number;
    pageSize: number;
}

export type PagedProcessResult = {
    type: "hierarchy";
    data: Paged<Process>;
} | {
    type: "list";
    data: Paged<Process>;
}

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