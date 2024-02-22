import { writable, type Writable } from "svelte/store";

export const search_string = writable('');

export const expandedSidenav = writable(false);

export function searchEmail(email: Data.Email, search_string: string) {
    return email.subject.toLowerCase().includes(search_string.toLowerCase()) ||
        email.sender.toLowerCase().includes(search_string.toLowerCase()) ||
        email.body.replace(/<\/?[^>]+(>|$)/g, "").replace(/&[^;]+;/g, " ").toLowerCase().includes(search_string.toLowerCase());
}

export const theme = writable('light');
export const lazyloadingenabled = writable(true);

export const logs: Writable<{ type: string, message: string }[]> = writable([]);