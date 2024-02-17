import { writable, type Writable } from 'svelte/store';

export const selected_previews: Writable<number[]> = writable([]);