import type { PageArgs } from "./misc";

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

export interface GetProcessArgs extends PageArgs {
    name: string | null;
    display: "list" | "hierarchy";
}

export interface GetProgramsArgs extends PageArgs {
    
}

export interface GetNetTableArgs extends PageArgs {
    protocols: Array<"tcp" | "udp">;
    processName: string | null;
    localPort: number | null;
    remotePort: number | null;
    localIpAddr: string | null;
    remoteIpAddr: string | null;
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
    pageSize: number;
    total: number;
    items: T[];
}

export interface Program {
    name: string;
    path: string;
}

export interface NetTableEntry {
    localIpAddress: string;
    localPort: number;
    processId: number;
    processName: string;
    remotePort: number;
    remoteIpAddress: string;
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