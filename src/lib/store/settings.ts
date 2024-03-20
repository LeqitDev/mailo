import { writable, type Writable } from "svelte/store";


export const expandedSidenav = writable(false);


export const readyCheck: Writable<{ ready: boolean, events_registered: boolean }> = writable({ ready: false, events_registered: false });

const defaultSettings: Data.Settings = {
    theme: 'light',
    lazyLoading: {
        enabled: true,
        amount: 25,
    },
    dashboardViewCount: 10,
    backendSettings: {
        masterpassword: false,
    },
    dashboardEmailFilter: "unseen"
}

export const settings = writable(defaultSettings);
