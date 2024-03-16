<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { MoonIcon, SunIcon, ArrowBigDownIcon, HomeIcon, MailIcon, PlusIcon, Settings2Icon, MoreVerticalIcon, UserCogIcon } from 'lucide-svelte';
	import CustomLayout from '../lib/custom/layouts/_customLayout.svelte';
	import { readyCheck, settings } from '@/stores/settings';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Label } from '@/components/ui/label';
	import { emailSortDateFunction, emails } from '@/stores/emails';
	import EmailPreview from '@/custom/components/EmailPreview.svelte';
	import { split } from 'postcss/lib/list';
	import { accounts } from '@/stores/accounts';
	import * as Avatar from '@/components/ui/avatar';
	import { cn } from '@/utils';
	import type { PageData } from './$types';
	import AddAccountDialog from '@/custom/components/AddAccountDialog.svelte';
	import EditAccountDialog from '@/custom/components/EditAccountDialog.svelte';

	export let data: PageData;
	console.log('data', data);

	function onThemeSelected(new_theme: string) {
		if (new_theme !== 'light' && new_theme !== 'dark') return;
		settings.update((settings) => {
			settings.theme = new_theme;
			return settings;
		});
	}

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

	function hashStr(str: string) {
		var hash = 0,
			i,
			chr;
		if (str.length === 0) return hash;
		for (i = 0; i < str.length; i++) {
			chr = str.charCodeAt(i);
			hash = (hash << 5) - hash + chr;
			hash |= 0; // Convert to 32bit integer
		}
		return hash;
	}

	function getLogParsed(log: Data.CustomEvent) {
		return `${(log.payload as Data.LogPayload).log_type}: ${(log.payload as Data.LogPayload).message}`;
	}

	function getEmailInitials(email: string) {
		return email
			.split('@')[0]
			.split('.')
			.map((part) => part[0])
			.join('');
	}

	$: recentEmails = $emails
		.sort(emailSortDateFunction)
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
					{#if recentEmails.length > 14}
						<p class="text-sm text-muted-foreground">{recentEmails.length - 14} more unseen emails hidden</p>
					{/if}
				</div>
				{#each recentEmails.slice(0, 14) as email, id (email.id)}
					<EmailPreview {email} {id} variant="compact" />
				{/each}
				<Button variant="link" href="/mail/inbox" class="mt-2">View all</Button>
			</div>
			<div class="lg:w-max w-full">
				<div class="px-4 pb-4 pt-1">
					<div class="mb-2 flex justify-between items-center">
						<p class="text-xl font-semibold">Manage your accounts</p>
						<AddAccountDialog data={data.add_account_form}>
							<Button variant="ghost" size="icon"><PlusIcon /></Button>
						</AddAccountDialog>
					</div>
					<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-1 lg:max-w-sm gap-2">
						{#each $accounts as account}
							<div class="flex items-center gap-2">
								<Avatar.Root>
									<Avatar.Image
										src="https://api.dicebear.com/7.x/shapes/svg?seed={hashStr(account.email)}"
										alt="Profile picture"
									/>
									<Avatar.Fallback>{getEmailInitials(account.email)}</Avatar.Fallback>
								</Avatar.Root>
								<div class="w-full">
									<div class="flex justify-between w-full">
										<div>
											<p class="font-semibold">{account.display_name}</p>
											<p class="text-muted-foreground overflow-ellipsis">{account.email}</p>
										</div>
										<EditAccountDialog form_data={data.account_settings_form} {account} >
											<Button variant="ghost" size="icon"><UserCogIcon /></Button>
										</EditAccountDialog>
									</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
				<div class="mt-4 rounded-sm px-4 pb-4 pt-1">
					<p class="mb-2 text-xl font-semibold">Change your theme</p>
					<RadioGroup.Root value={$settings.theme} class="" onValueChange={onThemeSelected}>
						<div class="flex space-x-2">
							<RadioGroup.Item value="light" id="light" />
							<Label for="light" class="cursor-pointer"
								>Light
								<div
									class="mt-2 h-20 w-80 overflow-clip rounded-sm border bg-background dark:border-muted-foreground dark:bg-foreground"
								>
									<div class="flex h-full gap-2">
										<div class="grid h-full gap-2 border-r p-2 dark:border-muted-foreground">
											<div
												class={cn(
													buttonVariants({
														variant: 'default',
														size: 'icon',
														className: 'dark:bg-background dark:text-foreground'
													})
												)}
											>
												<HomeIcon />
											</div>
											<div
												class={cn(
													buttonVariants({
														variant: 'ghost',
														size: 'icon',
														className: 'hover:bg-[hsl(0_0%_90.1%)] dark:text-background'
													})
												)}
											>
												<MailIcon />
											</div>
										</div>
										<div class="p-2">
											<h1
												class="text-hidden mb-4 w-full overflow-hidden whitespace-nowrap text-2xl font-semibold dark:text-background"
											>
												Welcome to your Dashboard.
											</h1>
										</div>
									</div>
								</div>
							</Label>
						</div>
						<div class="flex space-x-2">
							<RadioGroup.Item value="dark" id="dark" />
							<Label for="dark" class="cursor-pointer"
								>Dark
								<div
									class="mt-2 h-20 w-80 overflow-clip rounded-sm border bg-foreground dark:bg-background"
								>
									<div class="flex h-full gap-2">
										<div
											class="grid h-full gap-2 border-r border-muted-foreground p-2 dark:border-border"
										>
											<div
												class={cn(
													buttonVariants({
														variant: 'default',
														size: 'icon',
														className:
															'bg-background text-foreground hover:bg-background dark:bg-foreground dark:text-background'
													})
												)}
											>
												<HomeIcon />
											</div>
											<div
												class={cn(
													buttonVariants({
														variant: 'ghost',
														size: 'icon',
														className:
															'text-background hover:bg-[hsl(0_0%_14.9%)] hover:text-background dark:text-foreground'
													})
												)}
											>
												<MailIcon />
											</div>
										</div>
										<div class="p-2">
											<h1
												class="text-hidden mb-4 w-full overflow-hidden whitespace-nowrap text-2xl font-semibold text-background dark:text-foreground"
											>
												Welcome to your Dashboard.
											</h1>
										</div>
									</div>
								</div>
							</Label>
						</div>
					</RadioGroup.Root>
				</div>
			</div>
		</div>
	</div>
</CustomLayout
>
