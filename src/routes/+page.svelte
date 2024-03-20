<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { MoonIcon, SunIcon, ArrowBigDownIcon, HomeIcon, MailIcon, PlusIcon, Settings2Icon, MoreVerticalIcon, UserCogIcon } from 'lucide-svelte';
	import CustomLayout from '../components/RootLayout.svelte';
	import { readyCheck, settings } from '@/store';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Label } from '@/components/ui/label';
	import { emailSortDateFunction, emails } from '@/store';
	import EmailPreview from '§/components/EmailPreview.svelte';
	import { split } from 'postcss/lib/list';
	import { accounts } from '@/store';
	import * as Avatar from '@/components/ui/avatar';
	import { cn } from '@/utils';
	import type { PageData } from './$types';
	import AddAccountDialog from '§/components/AddAccountDialog.svelte';
	import EditAccountDialog from '§/components/EditAccountDialog.svelte';
	import ThemeSwitcher from '§/components/ThemeSwitcher.svelte';
	import AccountViewer from '§/components/AccountViewer.svelte';

	export let data: PageData;
	console.log('data', data);

	onMount(() => {
		if (!$readyCheck.ready) {
			invoke('ready')
				.catch((error) => {
					console.log('error', error);
				})
				.then(() => {
					console.log('frontend ready');
					invoke('start_all_imap_threads').then(() => {
						console.log('all imap threads started');
						readyCheck.update((readyCheck) => {
							readyCheck.ready = true;
							return readyCheck;
						});
					})
				});
		}
	});

	$: recentEmails = $emails
		.filter((email) => {
			if (email.flags.seen) return false;
			return true;
		});
</script>

<CustomLayout site="home">
	<!-- darkmode toggle -->
	<div class="overflow-y-auto p-2 flex-shrink w-full">
		<!-- Dashboard welcome phrase -->
		<h1 class="mb-4 text-3xl font-bold ml-4 mt-4">Welcome to your Dashboard.</h1>
		<div class="lg:flex w-full items-start gap-4">
			<div class="transparent flex-1 rounded-sm px-4 pb-4 pt-1 lg:w-screen w-full">
				<div class="flex justify-between items-center">
					<p class="mb-2 text-xl font-semibold">New & recent emails</p>
					{#if recentEmails.length > $settings.dashboardViewCount}
						<p class="text-sm text-muted-foreground">{recentEmails.length - $settings.dashboardViewCount} more unseen emails hidden</p>
					{/if}
				</div>
				{#each recentEmails.slice(0, $settings.dashboardViewCount) as email, id (email.id)}
					<EmailPreview {email} {id} variant="compact" />
				{/each}
				<Button variant="link" href="/mail/inbox" class="mt-2">View all</Button>
			</div>
			<div class="lg:w-max w-full">
				<div class="px-4 pb-4 pt-1">
					<AccountViewer {data} />
				</div>
				<div class="mt-4 rounded-sm px-4 pb-4 pt-1">
					<ThemeSwitcher />
				</div>
			</div>
		</div>
	</div>
</CustomLayout
>
