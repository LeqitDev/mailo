<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import {
		SearchIcon,
		InboxIcon,
		Trash2Icon,
		SendIcon,
		FileIcon,
		ArrowLeftFromLineIcon,
		ArrowRightFromLineIcon
	} from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';
	import * as Dialog from '$lib/components/ui/dialog';
	import { accounts, fetchAccounts } from '@/stores/accounts';
	import MailAccountForm from './MailAccountForm.svelte';
	import type { LayoutData } from './$types';
	import type { FormOptions } from 'formsnap';
	import { mailAccountSchema, type MailAccount } from './schema';
	import { invoke } from '@tauri-apps/api/tauri';
	import { expandedSidenav, search_string } from '@/stores/settings';
	import { selected_previews } from '@/stores/emails';
	import CustomLayout from '../_customLayout.svelte';
	import { fade, fly, slide } from 'svelte/transition';
	import { onMount } from 'svelte';

	export let data: LayoutData;

	var current_mailbox = 'inbox';
	var ready = false;

	onMount(() => {
		ready = true;
	});
</script>

<CustomLayout>
	<div slot="mail-sidebar">
		{#if $expandedSidenav && ready}
			<div class="grid gap-1 border-b" transition:slide>
				<Button
					variant={current_mailbox == 'inbox' ? 'secondary' : 'ghost'}
					size="sm"
					href="/mail/inbox"
					class="justify-start gap-2"
					on:click={() => {
						if (current_mailbox !== 'inbox') selected_previews.update(() => []);
						current_mailbox = 'inbox';
					}}><InboxIcon class="h-4 w-4" /> Inbox</Button
				>
				<Button
					variant={current_mailbox == 'sent' ? 'secondary' : 'ghost'}
					size="sm"
					href="/mail/sent"
					class="justify-start gap-2"
					on:click={() => {
						if (current_mailbox !== 'sent') selected_previews.update(() => []);
						current_mailbox = 'sent';
					}}><SendIcon class="h-4 w-4" /> Sent</Button
				>
				<Button
					variant={current_mailbox == 'drafts' ? 'secondary' : 'ghost'}
					size="sm"
					href="/mail/drafts"
					class="justify-start gap-2"
					on:click={() => {
						if (current_mailbox !== 'drafts') selected_previews.update(() => []);
						current_mailbox = 'drafts';
					}}><FileIcon class="h-4 w-4" /> Drafts</Button
				>
				<Button
					variant={current_mailbox == 'trash' ? 'secondary' : 'ghost'}
					size="sm"
					href="/mail/trash"
					class="justify-start gap-2"
					on:click={() => {
						if (current_mailbox !== 'trash') selected_previews.update(() => []);
						current_mailbox = 'trash';
					}}><Trash2Icon class="h-4 w-4" /> Trash</Button
				>
			</div>
		{:else if ready}
			<div class="grid gap-1 border-b" transition:slide>
				<Button
					variant={current_mailbox == 'inbox' ? 'secondary' : 'ghost'}
					size="icon"
					href="/mail/inbox"
					on:click={() => {
						if (current_mailbox !== 'inbox') selected_previews.update(() => []);
						current_mailbox = 'inbox';
					}}><InboxIcon class="h-4 w-4" /></Button
				>
				<Button
					variant={current_mailbox == 'sent' ? 'secondary' : 'ghost'}
					size="icon"
					href="/mail/sent"
					on:click={() => {
						if (current_mailbox !== 'sent') selected_previews.update(() => []);
						current_mailbox = 'sent';
					}}><SendIcon class="h-4 w-4" /></Button
				>
				<Button
					variant={current_mailbox == 'drafts' ? 'secondary' : 'ghost'}
					size="icon"
					href="/mail/drafts"
					on:click={() => {
						if (current_mailbox !== 'drafts') selected_previews.update(() => []);
						current_mailbox = 'drafts';
					}}><FileIcon class="h-4 w-4" /></Button
				>
				<Button
					variant={current_mailbox == 'trash' ? 'secondary' : 'ghost'}
					size="icon"
					href="/mail/trash"
					on:click={() => {
						if (current_mailbox !== 'trash') selected_previews.update(() => []);
						current_mailbox = 'trash';
					}}><Trash2Icon class="h-4 w-4" /></Button
				>
			</div>
		{/if}
	</div>
	<div class="flex min-h-screen w-full flex-col">
		<div class="select-none border-b px-2 pb-1 pt-2">
			<div class="flex justify-between">
				<Select.Root>
					<!-- selected={{value: "all", label: "All"}} -->
					<Select.Trigger class="max-w-sm" value="all">
						<Select.Value placeholder="Mail-Account" />
					</Select.Trigger>
					<Select.Content>
						{#if $accounts}
							<Select.Item value="all">All</Select.Item>
							{#each $accounts as account}
								<Select.Item value={account.id}>{account.email}</Select.Item>
							{/each}
						{/if}
					</Select.Content>
					<Select.Input value="all" />
				</Select.Root>
				<form class="flex w-full max-w-md items-center space-x-2">
					<Input on:input={(e) => search_string.update((v) => e.currentTarget.value)} />
					<Button variant="ghost" size="icon" on:click={() => invoke('ready')}
						><SearchIcon class="h-4 w-4" /></Button
					>
				</form>
				<div>
					<Button variant="outline" size="sm" on:click={() => fetchAccounts()}>Settings</Button>
					<Dialog.Root>
						<Dialog.Trigger class={buttonVariants({ variant: 'outline', size: 'sm' })}
							>Add Account</Dialog.Trigger
						>
						<Dialog.Content class="sm:max-w-[425px]">
							<Dialog.Header>
								<Dialog.Title>Add Account</Dialog.Title>
								<Dialog.Description>
									Add a new account with imap to your mail client.
								</Dialog.Description>
							</Dialog.Header>
							<MailAccountForm form={data.form} />
							<Dialog.Footer>
								<Button type="submit" form="mail-account-form">Save account</Button>
							</Dialog.Footer>
						</Dialog.Content>
					</Dialog.Root>
					<Button
						variant="outline"
						size="sm"
						on:click={() => invoke('logout').catch((e) => console.log('invoke error', e))}
						>Accounts</Button
					>
				</div>
			</div>
		</div>
		<div class="flex flex-grow overflow-hidden">
			<div class="w-full">
				<slot />
			</div>
		</div>
	</div>
</CustomLayout>
