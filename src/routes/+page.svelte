<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { MoonIcon, SunIcon, ArrowBigDownIcon, HomeIcon, MailIcon } from 'lucide-svelte';
	import CustomLayout from '../lib/custom/layouts/_customLayout.svelte';
	import { logs, readyCheck, theme } from '@/stores/settings';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Label } from '@/components/ui/label';
	import { emails } from '@/stores/emails';
	import EmailPreview from '@/custom/components/EmailPreview.svelte';
	import { split } from 'postcss/lib/list';
	import { accounts } from '@/stores/accounts';
	import * as Avatar from '@/components/ui/avatar';

	function onColorSwitchBtnClicked() {
		if ($theme === 'light') {
			theme.set('dark');
		} else {
			theme.set('light');
		}
	}

	function onThemeSelected(new_theme: string) {
		theme.set(new_theme);
	}

	onMount(() => {
		if (!$readyCheck.ready) {
			invoke('ready')
				.catch((error) => {
					console.log('error', error);
				})
				.then(() => {
					console.log('frontend ready');
					readyCheck.update((readyCheck) => {
						readyCheck.ready = true;
						return readyCheck;
					});
				});
		}
	});

	function hashStr(str: string) {
		var hash = 0,
			i, chr;
		if (str.length === 0) return hash;
		for (i = 0; i < str.length; i++) {
			chr = str.charCodeAt(i);
			hash = ((hash << 5) - hash) + chr;
			hash |= 0; // Convert to 32bit integer
		}
		return hash;
	}

	function getLogParsed(log: Data.EventPayload) {
		return `${(log.payload as Data.LogPayload).log_type}: ${(log.payload as Data.LogPayload).message}`;
	}

	function getEmailInitials(email: string) {
		return email.split('@')[0].split('.').map((part) => part[0]).join('');
	}

	$: recentEmails = $emails.filter((email) => {
		if (email.flags.seen) return false;
		return true;
	}).reverse();
</script>

<CustomLayout>
	<!-- darkmode toggle -->
	<div class="p-2 overflow-y-auto">
		<!-- Dashboard welcome phrase -->
		<h1 class="mb-4 text-2xl font-semibold">Welcome to your Dashboard.</h1>
		<div class="lg:flex gap-4 w-full items-start">
			<div class="rounded-sm shadow-sm shadow-blue-500 transparent px-4 pb-4 pt-1 lg:flex-grow max-w-5xl">
				<p class="mb-2 text-lg font-semibold">New & recent emails</p>
				{#each recentEmails.slice(0, 4) as email, id (email.id)}
					<EmailPreview {email} {id} variant="compact" />
				{/each}
				<Button variant="link" href="/mail/inbox" class="mt-2">View all</Button>
			</div>
			<div class="rounded-sm shadow-sm shadow-blue-500 mt-4 lg:mt-0 px-4 pb-4 pt-1">
				<p class="mb-2 text-lg font-semibold">Manage your theme</p>
				<RadioGroup.Root value={$theme} class="" onValueChange={onThemeSelected}>
					<div class="flex space-x-2">
						<RadioGroup.Item value="light" id="light" />
						<Label for="light" class="cursor-pointer"
							>Light
							<div
								class="mt-2 h-20 w-80 overflow-clip rounded-sm border bg-background dark:border-muted-foreground dark:bg-foreground"
							>
								<div class="flex h-full gap-2">
									<div class="grid h-full gap-2 border-r p-2 dark:border-muted-foreground">
										<Button
											size="icon"
											variant="default"
											class="dark:bg-background dark:text-foreground"><HomeIcon /></Button
										>
										<Button size="icon" variant="ghost" class="dark:text-background"
											><MailIcon /></Button
										>
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
										<Button
											size="icon"
											variant="default"
											class="bg-background text-foreground dark:bg-foreground dark:text-background"
											><HomeIcon /></Button
										>
										<Button size="icon" variant="ghost" class="text-background dark:text-foreground"
											><MailIcon /></Button
										>
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
		<div class="lg:flex mt-4">
			<div class="rounded-sm shadow-sm shadow-blue-500 px-4 pb-4 pt-1max-w-5xl">
				<p class="mb-2 text-lg font-semibold">Manage your accounts</p>
				<div class="grid grid-cols-[auto_auto]">
					{#each $accounts as account}
						<div class="flex gap-2 items-center">
							<Avatar.Root>
								<Avatar.Image src="https://api.dicebear.com/7.x/shapes/svg?seed={hashStr(account.email)}"  alt="Profile picture"/>
								<Avatar.Fallback>{getEmailInitials(account.email)}</Avatar.Fallback>
							</Avatar.Root>
							<div>
								<p class="font-semibold">E-Mail: {account.email}</p>
								<Button variant="default" size="sm">Manage</Button>
							</div>
						</div>
					{/each}
				</div>
				<!-- Add new account -->
				<Button variant="link" href="/mail/accounts/add" class="mt-2">Add new account</Button>
			</div>
		</div>
	</div>
	<!-- {#if $logs}
		<div class="p-2">
			{#each $logs as log}
				<div class="rounded-lg bg-gray-200 p-2 dark:bg-gray-800">
					<p>{getLogParsed(log)}</p>
				</div>
			{/each}
		</div>
	{/if} -->
</CustomLayout>
