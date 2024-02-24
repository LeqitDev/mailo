import { invoke } from '@tauri-apps/api/tauri';
import { writable, type Writable } from 'svelte/store';

export const selected_previews: Writable<number[]> = writable([]);
export const emails: Writable<Data.Email[]> = writable([]);

fetchEmails();

export async function fetchEmails() {
    /* let window = null;
    window = (await import('@tauri-apps/api/tauri')).default;
    window?. */invoke('get_emails').then((fetched_emails) => {
        emails.set(fetched_emails as Data.Email[]);
        // console.log('fetched_emails', fetched_emails as Data.Email[]);
    }).catch((error) => {
        console.log('error fetching emails', error);
    });
}