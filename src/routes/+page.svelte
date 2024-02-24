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

	function getLogParsed(log: Data.EventPayload) {
		return `${(log.payload as Data.LogPayload).log_type}: ${(log.payload as Data.LogPayload).message}`;
	}

	$: recentEmails = $emails.filter((email) => {
		if (email.flags.seen) return false;
		return true;
	});
</script>

<CustomLayout>
	<!-- darkmode toggle -->
	<div class="p-2">
		<!-- Dashboard welcome phrase -->
		<h1 class="mb-4 text-2xl font-semibold">Welcome to your Dashboard.</h1>
		<div class="grid grid-cols-[minmax(40rem,_1fr)_auto] gap-8">
			<div class="rounded-sm border px-4 pb-4 pt-1">
				<p class="mb-2 text-lg font-semibold">New & recent emails</p>
				{#each recentEmails as email, id (email.id)}
					<EmailPreview {email} {id} />
				{/each}
			</div>
			<div class="rounded-sm border px-4 pb-4 pt-1">
				<p class="mb-2 text-lg font-semibold">Manage your theme</p>
				<RadioGroup.Root value={$theme} class="" onValueChange={onThemeSelected}>
					<div class="flex space-x-2">
						<RadioGroup.Item value="light" id="light" />
						<Label for="light"
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
						<Label for="dark"
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
		<Button
			size="icon"
			on:click={() =>
				invoke('add_event', { eventType: 'action', payload: 'fetch_emails' })
					.catch((e) => console.log(e))
					.then(() => console.log('ok'))}><ArrowBigDownIcon /></Button
		>
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
