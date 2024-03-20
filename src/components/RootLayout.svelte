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
	import { events, expandedSidenav, initialize_events, readyCheck, settings } from '@/store';
	import { Toaster } from '@/components/ui/sonner';
	import { toast } from 'svelte-sonner';
	import { fetchEmails } from '@/store';
	import { browser } from '$app/environment';
	import SideNavButton from '§/components/SideNavButton.svelte';

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
		const new_logs = (await invoke('fetch_logs')) as Data.LogPayload[];
		if (new_logs.length > 0) {
			console.log('logs', new_logs);
			const log_events: Data.CustomEvent[] = [];
			for (const log of new_logs) {
				toast(log.message);
				log_events.push({
					event: 'log',
					payload: log,
					time: new Date().getTime().toString()
				});
			}
			events.update((events) => {
				events = log_events.concat(events);
				return events;
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
				settings.update((settings) => {
					settings.theme = 'dark';
					return settings;
				});
				document.documentElement.classList.add('dark');
			} else if (
				!('theme' in localStorage) &&
				window.matchMedia('(prefers-color-scheme: dark)').matches
			) {
				settings.update((settings) => {
					settings.theme = 'dark';
					return settings;
				});
				document.documentElement.classList.add('dark');
			} else {
				document.documentElement.classList.remove('dark');
				settings.update((settings) => {
					settings.theme = 'light';
					return settings;
				});
			}
		}

		/* settings.subscribe((value) => {
			console.log('theme', value);
			localStorage.setItem('theme', value);
			console.log('storedTheme', localStorage.theme);
			if (value !== 'dark') {
				document.documentElement.classList.remove('dark');
			}
		}); */

		settings.subscribe((settings) => {
			localStorage.setItem('theme', settings.theme);
			if (settings.theme === 'dark') {
				document.documentElement.classList.add('dark');
			} else {
				document.documentElement.classList.remove('dark');
			}
		});

		if (!$readyCheck.events_registered) {
			initialize_events();
			invoke('get_settings').then((backend_settings) => {
				console.log('settings', settings);
				settings.update((settings) => {
					settings.backendSettings = backend_settings as Data.BackendSettings;
					return settings;
				});
			});
		}
	});
</script>

<Toaster />

<main class="flex h-full w-full">
	<div class="flex h-full flex-col gap-2 border-r p-1 pt-2" class:min-w-60={$expandedSidenav}>
		<SideNavButton {site} bind:expanded={$expandedSidenav} name="Home" site_name="home" href="/"><HomeIcon /></SideNavButton>
		<SideNavButton {site} bind:expanded={$expandedSidenav} name="Mail" site_name="mail" href="/mail/inbox"
			><MailIcon /></SideNavButton
		>
		<slot name="mail-sidebar" />
		<SideNavButton {site} bind:expanded={$expandedSidenav} name="Calendar" site_name="calendar" href="/calendar"
			><CalendarIcon /></SideNavButton
		>
		<SideNavButton {site} bind:expanded={$expandedSidenav} name="Settings" site_name="settings" href="/settings"
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
