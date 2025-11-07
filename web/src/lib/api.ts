import { invoke } from "@tauri-apps/api/core";
import type { LoadResult, Process, UpdateStatus } from "./types";
import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";

export const loadApp = (): Promise<LoadResult> => invoke("load");

export const checkUpdates = (install: boolean): Promise<void> => emit("check-updates", { install });

export const onUpdateStatusChange = (handler: (value: UpdateStatus) => void): Promise<UnlistenFn> => listen<UpdateStatus>("on-update", (event) => handler(event.payload));

export interface GetProcessArgs {
    name: string | null;
    display: "list" | "hierarchy"
}

export const getProcesses = (args: GetProcessArgs): Promise<any> => invoke("get_processes", { args });

export const getProcessById = (id: number): Promise<any> => invoke("get_process", { id });