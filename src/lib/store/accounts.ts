import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

export const accounts: Writable<Data.Account[]> = writable([]);
export const selected_account: Writable<Data.Account | null> = writable(null);

fetchAccounts();

export async function fetchAccounts() {
    invoke('get_accounts').then((fetched_accounts) => {
        (fetched_accounts as Data.Account[]).forEach((account) => {
            if (!account.display_name) {
                account.display_name = getPotentialName(account.email);
            }
        });
        accounts.set(fetched_accounts as Data.Account[]);
        console.log('fetched_accounts', fetched_accounts as Data.Account[]);
    }).catch((error) => {
        console.log('error', error);
    });
}

function getPotentialName(email: string) {
    return email
        .split('@')[0]
        .split('.')
        .map((part) => part[0].toUpperCase() + part.slice(1))
        .join(' ');
}