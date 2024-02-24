<script	lang="ts">
	import { Button } from '@/components/ui/button';
	import '§/app.pcss';
	import { ArrowLeftFromLine, ArrowRightFromLine, CalendarIcon, HomeIcon, MailIcon, SchoolIcon } from 'lucide-svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke } from '@tauri-apps/api';
    import * as tauri_window from '@tauri-apps/api/window';
	import { attachConsole } from 'tauri-plugin-log-api';
    import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { expandedSidenav, initialize_events, logs, readyCheck, theme } from '@/stores/settings';
	import { Toaster } from '@/components/ui/sonner';
	import { toast } from 'svelte-sonner';
	import { fetchEmails } from '@/stores/emails';
	import { browser } from '$app/environment';

    

    tauri_window.getCurrent().listen('tauri://close-requested', () => {
        console.log('close-requested');
        toast.loading('Closing sessions, saving data...');
        invoke('logout').catch((error) => {
            console.log('error', error);
        });
    });

    fetch_logs();

    async function fetch_logs() {
        const new_logs = await invoke('fetch_logs') as Data.EventPayload[];
        if (new_logs.length > 0) {
            console.log('logs', new_logs);
            for (const log of new_logs) {
                toast((log.payload as Data.LogPayload).message); 
            }
            logs.update((old_logs) => {
                return [...old_logs, ...new_logs];
            });
        }
        // setTimeout(fetch_logs, 2000);
    }

    const detach_log = attachConsole();
	var ready = false;

    if (!$readyCheck.events_registered) {
        initialize_events();
    }

	onMount(() => {
		ready = true;

        if (browser) {
            if (localStorage.theme === 'dark') {
                theme.set('dark');
                document.documentElement.classList.add('dark');
            } else if (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches) {
                theme.set('dark');
                document.documentElement.classList.add('dark');
            } else {
                document.documentElement.classList.remove('dark');
                theme.set('light');
            }
        }
	});

	theme.subscribe((value) => {
        localStorage.setItem('theme', value);
		if (value === 'dark') {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	});
</script>

<Toaster />

<main class="w-full h-full flex">
    {#if $expandedSidenav}
        <div class="h-full border-r p-1 pt-2 flex flex-col gap-2 w-60">
            <Button variant={!($page.url.pathname.includes("mail")) ? "default" : "ghost"} href="/" class="justify-start gap-2"><HomeIcon /> Home</Button>
            <Button variant={$page.url.pathname.includes("mail") ? "default" : "ghost"} href="/mail/inbox" class="justify-start gap-2"><MailIcon /> E-Mail</Button>
            <div class="pl-6">
                <slot name="mail-sidebar"></slot>
            </div>
            <Button variant="ghost" class="justify-start gap-2"><CalendarIcon /> Calendar</Button>
            <Button variant="ghost" size="icon" class="mt-auto ml-auto" on:click={() => expandedSidenav.set(!$expandedSidenav)}><ArrowLeftFromLine /></Button>
        </div>
    {:else}
        <div class="h-full border-r p-1 pt-2 flex flex-col gap-2">
            <Button variant={!($page.url.pathname.includes("mail")) ? "default" : "ghost"} size="icon" href="/"><HomeIcon /></Button>
            <Button variant={$page.url.pathname.includes("mail") ? "default" : "ghost"} size="icon" href="/mail/inbox"><MailIcon /></Button>
            <slot name="mail-sidebar"></slot>
            <Button variant="ghost" size="icon"><CalendarIcon /></Button>
            <Button variant="ghost" size="icon" class="mt-auto" on:click={() => expandedSidenav.set(!$expandedSidenav)}><ArrowRightFromLine /></Button>
        </div>
    {/if}
    <slot>
    </slot>
</main>