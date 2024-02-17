import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

export const accounts: Writable<Data.Account[]> = writable([]);

await fetchAccounts();

export async function fetchAccounts() {
    invoke('get_accounts').then((fetched_accounts) => {
        accounts.set(fetched_accounts as Data.Account[]);
        console.log('fetched_accounts', fetched_accounts as Data.Account[]);
    }).catch((error) => {
        console.log('error', error);
    });
}