<script	lang="ts">
	import { Button } from '@/components/ui/button';
	import '../../../app.pcss';
	import { ArrowLeftFromLine, ArrowRightFromLine, CalendarIcon, HomeIcon, MailIcon, SchoolIcon } from 'lucide-svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke, window } from '@tauri-apps/api';
	import { attachConsole } from 'tauri-plugin-log-api';
    import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { cubicInOut, cubicOut } from 'svelte/easing';
	import { fade, slide, type TransitionConfig } from 'svelte/transition';
	import { number } from 'zod';
	import { expandedSidenav, logs } from '@/stores/settings';

    const unlisten_log = listen('log', (event) => {
        console.log('event', event);
    });
    window.getCurrent().listen('tauri://close-requested', () => {
        console.log('close-requested');
        invoke('logout').catch((error) => {
            console.log('error', error);
        });
    });

    fetch_logs();

    async function fetch_logs() {
        const new_logs = await invoke('fetch_logs') as { type: string, message: string }[];
        if (new_logs.length > 0) {
            console.log('logs', new_logs);
            logs.update((old_logs) => {
                return [...old_logs, ...new_logs];
            });
        }
        setTimeout(fetch_logs, 2000);
    }

    const detach_log = attachConsole();
	var ready = false;

	onMount(() => {
		ready = true;
	});
</script>

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