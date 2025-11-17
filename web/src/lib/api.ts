import { invoke } from "@tauri-apps/api/core";
import type { DashboardStats, GetProcessArgs, GetProgramsArgs, LoadResult, Paged, PagedProcessResult, Process, Program, UpdateStatus } from "./types";
import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export const loadApp = (): Promise<LoadResult> => invoke("load");

export const minimize = async (): Promise<void> => {
    const window = getCurrentWebviewWindow();
    await window.minimize();
};

export const checkUpdates = (install: boolean): Promise<void> => emit("check-updates", { install });

export const onUpdateStatusChange = (handler: (value: UpdateStatus) => void): Promise<UnlistenFn> => listen<UpdateStatus>("on-update", (event) => handler(event.payload));

export const onScreenshotRequest = (handler: () => void): Promise<UnlistenFn> => listen<UpdateStatus>("on-screenshot", (event) => handler());

export const getProcesses = (args: GetProcessArgs): Promise<PagedProcessResult> => invoke("get_processes", { args });

export const getDashboardStats = (): Promise<DashboardStats> => invoke("get_dashboard_stats");

export const getPrograms = (args: GetProgramsArgs): Promise<Paged<Program>> => invoke("get_programs", { args });

export const getProgramsCount = (): Promise<any> => invoke("get_programs_count");

export const getInstalledDrivers = (): Promise<any> => invoke("get_installed_drivers");

export const getLoadedDrivers = (): Promise<any> => invoke("get_loaded_drivers");

export const getMemoryInfo = (): Promise<any> => invoke("get_memory_info");

export const getSystemHandles = (): Promise<any> => invoke("get_system_handles");

export const saveScreenshot = (dataUrl: string): Promise<never> => invoke("save_screenshot", { dataUrl });

export const requestScreenshot = (): Promise<void> => emit("on-screenshot");

export const getProcessById = (id: number): Promise<any> => invoke("get_process", { id });

export const killProcess = (id: number): Promise<any> => invoke("kill_process", { id });