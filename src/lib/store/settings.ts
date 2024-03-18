import { writable, type Writable } from "svelte/store";


export const expandedSidenav = writable(false);


export const readyCheck: Writable<{ ready: boolean, events_registered: boolean }> = writable({ ready: false, events_registered: false });

const defaultSettings: Data.Settings = {
    theme: 'light',
    lazyLoadingEnabled: true,
    backendSettings: {
        masterpassword: false,
    },
    dashboardEmailFilter: "unseen"
}

export const settings = writable(defaultSettings);
