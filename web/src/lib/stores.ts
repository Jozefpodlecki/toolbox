import { writable } from "svelte/store";
import type { UpdateStatus } from "./types";

export const updateStatus = writable<UpdateStatus>({type: "idle"});