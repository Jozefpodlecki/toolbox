import { invoke } from "@tauri-apps/api/core";
import type { LoadResult, Paged, Process, Program, UpdateStatus } from "./types";
import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";

export const loadApp = (): Promise<LoadResult> => invoke("load");

export const checkUpdates = (install: boolean): Promise<void> => emit("check-updates", { install });

export const onUpdateStatusChange = (handler: (value: UpdateStatus) => void): Promise<UnlistenFn> => listen<UpdateStatus>("on-update", (event) => handler(event.payload));

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

export const getProcesses = (args: GetProcessArgs): Promise<Paged<Process>> => invoke("get_processes", { args });

export const getPrograms = (args: GetProgramsArgs): Promise<Paged<Program>> => invoke("get_programs", { args });

export const getProgramsCount = (): Promise<any> => invoke("get_programs_count");

export const getProcessById = (id: number): Promise<any> => invoke("get_process", { id });