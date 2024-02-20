<script	lang="ts">
	import { Button } from '@/components/ui/button';
	import '../app.pcss';
	import { CalendarIcon, HomeIcon, MailIcon } from 'lucide-svelte';
	import { listen } from '@tauri-apps/api/event';
	import { invoke, window } from '@tauri-apps/api';
	import { attachConsole } from 'tauri-plugin-log-api';
    import { page } from '$app/stores';

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
        const logs = await invoke('fetch_logs') as { type: string, message: string }[];
        if (logs.length > 0) {
            console.log('logs', logs);
        }
        setTimeout(fetch_logs, 2000);
    }

    const detach_log = attachConsole();
</script>

<main class="w-full h-full flex">
    <div class="h-full border-r p-1 pt-2 flex flex-col gap-2">
        <Button variant={!($page.url.pathname.includes("mail")) ? "outline" : "ghost"} size="icon" href="/"><HomeIcon /></Button>
        <Button variant={$page.url.pathname.includes("mail") ? "outline" : "ghost"} size="icon" href="/mail/inbox"><MailIcon /></Button>
        <Button variant="ghost" size="icon"><CalendarIcon /></Button>
    </div>
	<slot />
</main>