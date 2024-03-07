<script lang="ts">
	import { Button } from '@/components/ui/button';
	import '§/app.pcss';
	import {
		ArrowLeftFromLine,
		ArrowRightFromLine,
		CalendarIcon,
		HomeIcon,
		MailIcon,
		SchoolIcon,
		SettingsIcon
	} from 'lucide-svelte';
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
	import SideNavButton from '../components/SideNavButton.svelte';

	export let site: string;

	tauri_window.getCurrent().listen('tauri://close-requested', () => {
		console.log('close-requested');
		toast.loading('Closing sessions, saving data...');
		invoke('logout').catch((error) => {
			console.log('error', error);
		});
	});

	fetch_logs();

	async function fetch_logs() {
		const new_logs = (await invoke('fetch_logs')) as Data.EventPayload[];
		if (new_logs.length > 0) {
			console.log('logs', new_logs);
			for (const log of new_logs) {
				if (log && log.payload && log.payload.hasOwnProperty('message')) {
					toast((log.payload as Data.LogPayload).message);
				}
			}
			logs.update((old_logs) => {
				return [...old_logs, ...new_logs];
			});
		}
		// setTimeout(fetch_logs, 2000);
	}

	const detach_log = attachConsole();
	var ready = false;

	onMount(() => {
		ready = true;

		if (browser) {
			console.log('storedTheme', localStorage.theme);
			if (localStorage.theme === 'dark') {
				theme.set('dark');
				document.documentElement.classList.add('dark');
			} else if (
				!('theme' in localStorage) &&
				window.matchMedia('(prefers-color-scheme: dark)').matches
			) {
				theme.set('dark');
				document.documentElement.classList.add('dark');
			} else {
				document.documentElement.classList.remove('dark');
				theme.set('light');
			}
		}

		theme.subscribe((value) => {
			console.log('theme', value);
			localStorage.setItem('theme', value);
			console.log('storedTheme', localStorage.theme);
			if (value !== 'dark') {
				document.documentElement.classList.remove('dark');
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

		if (!$readyCheck.events_registered) {
			initialize_events();
		}
	});
</script>

<Toaster />

<main class="flex h-full w-full">
	<div class="flex h-full flex-col gap-2 border-r p-1 pt-2" class:min-w-60={$expandedSidenav}>
		<SideNavButton {site} name="Home" site_name="home" href="/"><HomeIcon /></SideNavButton>
		<SideNavButton {site} name="Mail" site_name="mail" href="/mail/inbox"
			><MailIcon /></SideNavButton
		>
		<div class:pl-6={$expandedSidenav}>
			<slot name="mail-sidebar" />
		</div>
		<SideNavButton {site} name="Calendar" site_name="calendar" href="/calendar"
			><CalendarIcon /></SideNavButton
		>
		<SideNavButton {site} name="Settings" site_name="settings" href="/settings"
			><SettingsIcon /></SideNavButton
		>
		<Button
			variant="ghost"
			size="icon"
			class={`mt-auto ${$expandedSidenav ? ' ml-auto' : ''}`}
			on:click={() => expandedSidenav.set(!$expandedSidenav)}
			>{#if $expandedSidenav}<ArrowLeftFromLine />{:else}<ArrowRightFromLine />{/if}</Button
		>
	</div>
	<slot />
</main>
