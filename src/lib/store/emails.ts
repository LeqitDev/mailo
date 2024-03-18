import { invoke } from '@tauri-apps/api/tauri';
import { writable, type Writable } from 'svelte/store';

export const selected_previews: Writable<number[]> = writable([]);
export const emails: Writable<Data.Email[]> = writable([]);

fetchEmails();

export async function fetchEmails() {
    /* let window = null;
    window = (await import('@tauri-apps/api/tauri')).default;
    window?. */invoke('get_emails').then((fetched_emails) => {
        emails.set((fetched_emails as Data.Email[]).sort(emailSortDateFunction));
        // console.log('fetched_emails', fetched_emails as Data.Email[]);
    }).catch((error) => {
        console.log('error fetching emails', error);
    });
}

export function emailSortDateFunction(a: Data.Email, b: Data.Email) {
    return parseDate(b.date).getTime() - parseDate(a.date).getTime();
}

function parseDate(date: string) {
    // parse "2024-2-3 20:8:9" to date object
    const [datePart, timePart] = date.split(' ');
    const [year, month, day] = datePart.split('-');
    const [hour, minute, second] = timePart.split(':');
    return new Date(Number(year), Number(month) - 1, Number(day), Number(hour), Number(minute), Number(second));
}