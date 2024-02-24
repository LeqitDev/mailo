import { listen } from "@tauri-apps/api/event";
import { toast } from "svelte-sonner";
import { writable, type Writable } from "svelte/store";
import { emails, fetchEmails } from "./emails";
import { browser } from "$app/environment";

export const search_string = writable('');

export const expandedSidenav = writable(false);

export function searchEmail(email: Data.Email, search_string: string) {
    return email.subject.toLowerCase().includes(search_string.toLowerCase()) ||
        email.sender.toLowerCase().includes(search_string.toLowerCase()) ||
        email.body.replace(/<\/?[^>]+(>|$)/g, "").replace(/&[^;]+;/g, " ").toLowerCase().includes(search_string.toLowerCase());
}

export const theme = writable('light');

theme.subscribe((value) => {
    console.log('theme', value);
    localStorage.setItem('theme', value);
    if (value === 'dark') {
    } else {
        document.documentElement.classList.remove('dark');
    }
});

export const lazyloadingenabled = writable(true);

export const logs: Writable<Data.EventPayload[]> = writable([]);
export const readyCheck: Writable<{ ready: boolean, events_registered: boolean }> = writable({ ready: false, events_registered: false });

export async function initialize_events() {
        const unlisten_log = await listen('log', (raw_event) => {
            let event = raw_event as unknown as Data.EventPayload;
            toast.info((event.payload as Data.LogPayload).message);
            console.log('event', event);
        });

        const unlisten_action = await listen('action', (raw_event) => {
            let event = raw_event as unknown as Data.EventPayload;
            console.log('event', event);
            toast.message('Action requested', { description: (event.payload as Data.ActionPayload).action})
            if ((event.payload as Data.ActionPayload).action === 'fetch_emails') {
                fetchEmails();
            }
        });

    readyCheck.update((value) => {
        value.events_registered = true;
        return value;
    });
}