import { listen } from "@tauri-apps/api/event";
import { toast } from "svelte-sonner";
import { writable, type Writable } from "svelte/store";
import { fetchEmails } from "./emails";
import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/api/notification";
import { readyCheck } from "./settings";

export const events: Writable<Data.CustomEvent[]> = writable([]);

export async function initialize_events() {
        await listen('log', (raw_event) => {
            const event = raw_event as unknown as Data.CustomEvent;
            toast.info((event.payload as Data.LogPayload).message);
            console.log('event-log', event);
            events.update((value) => {
                value.push({ event: "log", payload: event.payload, time: new Date().getTime().toString() });
                return value;
            });
        });

        await listen('action', (raw_event) => {
            const event = raw_event as unknown as Data.CustomEvent;
            console.log('event-action', event);
            toast.message('Action requested', { description: (event.payload as Data.ActionPayload).action})
            if ((event.payload as Data.ActionPayload).action === 'fetch_emails') {
                fetchEmails();
            }
            events.update((value) => {
                value.push({ event: "action", payload: event.payload, time: new Date().getTime().toString() });
                return value;
            });
        });

        await listen('notify', async (raw_event) => {
            console.log('event-notification', raw_event);
            let permissionGranted = await isPermissionGranted();
            if (!permissionGranted) {
                const permission = await requestPermission();
                permissionGranted = permission === 'granted';
            }
            if (permissionGranted) {
                const event = raw_event as unknown as Data.CustomEvent;
                const payload = event.payload as Data.NotifyPayload;
                sendNotification({title: payload.title, body: payload.body, sound: 'default', icon: '/favicon.ico'});
                events.update((value) => {
                    value.push({ event: "notify", payload: event.payload, time: new Date().getTime().toString() });
                    return value;
                });
            }
        });

    readyCheck.update((value) => {
        value.events_registered = true;
        return value;
    });
}