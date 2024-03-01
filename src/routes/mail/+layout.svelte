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
	import { accounts, fetchAccounts, selected_account } from '@/stores/accounts';
	import MailAccountForm from '$lib/custom/components/MailAccountForm.svelte';
	import type { LayoutData } from './$types';
	import { invoke } from '@tauri-apps/api/tauri';
	import { expandedSidenav, search_string } from '@/stores/settings';
	import { selected_previews } from '@/stores/emails';
	import CustomLayout from '$lib/custom/layouts/_customLayout.svelte';
	import { fade, fly, slide } from 'svelte/transition';
	import { onMount } from 'svelte';
	import SideNavButton from '@/custom/components/SideNavButton.svelte';

	export let data: LayoutData;

	var current_mailbox = 'inbox';
	var ready = false;

	var openAddAccountDialog = false;

	onMount(() => {
		ready = true;
	});
</script>

<CustomLayout site="mail">
	<div slot="mail-sidebar">
		{#if ready}
			<div class="grid gap-1 border-b pb-2" transition:slide>
				<SideNavButton site={current_mailbox} name="Inbox" site_name="inbox" size="sm" href="/mail/inbox" active_variant="secondary" clicked={() => {
					if (current_mailbox !== 'inbox') selected_previews.update(() => []);
					current_mailbox = 'inbox';
				}}><InboxIcon class="h-4 w-4" /></SideNavButton>
				<SideNavButton site={current_mailbox} name="Sent" site_name="sent" size="sm" href="/mail/sent" active_variant="secondary" clicked={() => {
					if (current_mailbox !== 'sent') selected_previews.update(() => []);
					current_mailbox = 'sent';
				}}><SendIcon class="h-4 w-4" /></SideNavButton>
				<SideNavButton site={current_mailbox} name="Drafts" site_name="drafts" size="sm" href="/mail/drafts" active_variant="secondary" clicked={() => {
					if (current_mailbox !== 'drafts') selected_previews.update(() => []);
					current_mailbox = 'drafts';
				}}><FileIcon class="h-4 w-4" /></SideNavButton>
				<SideNavButton site={current_mailbox} name="Trash" site_name="trash" size="sm" href="/mail/trash" active_variant="secondary" clicked={() => {
					if (current_mailbox !== 'trash') selected_previews.update(() => []);
					current_mailbox = 'trash';
				}}><Trash2Icon class="h-4 w-4" /></SideNavButton>
			</div>
		{/if}
	</div>
	<div class="flex min-h-screen w-full flex-col">
		<div class="select-none px-2 pb-1 pt-2">
			<div class="flex justify-between">
				<Select.Root selected={{value: "default", label: $accounts.length > 0 ? "All" : "No accounts"}} onSelectedChange={(selectedValue) => selected_account.set($accounts.find((value) => value.id === selectedValue?.value) ?? null)}>
					<Select.Trigger class="max-w-sm" value="default">
						<Select.Value placeholder="Select Mailaccount" />
					</Select.Trigger>
					<Select.Content>
						{#if $accounts.length > 0}
							<Select.Item value="default">All</Select.Item>
							{#each $accounts as account}
								<Select.Item value={account.id}>{account.email}</Select.Item>
							{/each}
						{:else}
							<Select.Item value="default">No accounts</Select.Item>
						{/if}
					</Select.Content>
					<Select.Input value="all" />
				</Select.Root>
				<form class="flex w-full max-w-md items-center space-x-2">
					<Input on:input={(e) => search_string.update((v) => e.currentTarget.value)} />
					<Button variant="ghost" size="icon" on:click={() => invoke('ready').then(() => console.log('ready'))}
						><SearchIcon class="h-4 w-4" /></Button
					>
				</form>
				<div>
					<Button variant="outline" size="sm" on:click={() => fetchAccounts()}>Settings</Button>
					<Dialog.Root bind:open={openAddAccountDialog}>
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
							<MailAccountForm form={data.form} onSuccessfulSubmit={() => {openAddAccountDialog = false}} />
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
