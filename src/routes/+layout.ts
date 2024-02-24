import { fetchEmails } from "@/stores/emails";
import { listen } from "@tauri-apps/api/event";
import { toast } from "svelte-sonner";

export const prerender = true
export const ssr = false
