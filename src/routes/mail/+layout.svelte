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

	export let data: LayoutData;
	console.log('data', data.form);

	var current_mailbox = 'inbox';
	var collapsed = true;
</script>

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
				<Input />
				<Button variant="ghost" size="icon"><SearchIcon class="h-4 w-4" /></Button>
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
				<Button variant="outline" size="sm">Accounts</Button>
			</div>
		</div>
	</div>
	<div class="flex flex-grow overflow-hidden">
		{#if collapsed}
			<div class="select-none border-r p-2">
				<Button variant="ghost" size="icon" class="mb-2" on:click={() => (collapsed = false)}
					><ArrowRightFromLineIcon class="h-5 w-5" /></Button
				>
				<div class="grid gap-1 border-t pt-4">
					<Button
						variant={current_mailbox == 'inbox' ? 'outline' : 'ghost'}
						size="icon"
						href="/mail/inbox"
						on:click={() => (current_mailbox = 'inbox')}><InboxIcon class="h-4 w-4" /></Button
					>
					<Button
						variant={current_mailbox == 'sent' ? 'outline' : 'ghost'}
						size="icon"
						href="/mail/sent"
						on:click={() => (current_mailbox = 'sent')}><SendIcon class="h-4 w-4" /></Button
					>
					<Button
						variant={current_mailbox == 'drafts' ? 'outline' : 'ghost'}
						size="icon"
						href="/mail/drafts"
						on:click={() => (current_mailbox = 'drafts')}><FileIcon class="h-4 w-4" /></Button
					>
					<Button
						variant={current_mailbox == 'trash' ? 'outline' : 'ghost'}
						size="icon"
						href="/mail/trash"
						on:click={() => (current_mailbox = 'trash')}><Trash2Icon class="h-4 w-4" /></Button
					>
				</div>
			</div>
		{:else}
			<div class="w-full max-w-48 select-none border-r p-4 lg:max-w-sm">
				<div class="flex items-center justify-between pb-4">
					<h2 class="text-xl font-semibold">Mail</h2>
					<Button variant="ghost" size="icon" on:click={() => (collapsed = true)}
						><ArrowLeftFromLineIcon class="h-5 w-5" /></Button
					>
				</div>
				<div class="grid gap-1">
					<Button
						variant={current_mailbox == 'inbox' ? 'outline' : 'ghost'}
						size="sm"
						class="justify-start gap-2"
						href="/mail/inbox"
						on:click={() => (current_mailbox = 'inbox')}><InboxIcon class="h-4 w-4" />Inbox</Button
					>
					<Button
						variant={current_mailbox == 'sent' ? 'outline' : 'ghost'}
						size="sm"
						class="justify-start gap-2"
						href="/mail/sent"
						on:click={() => (current_mailbox = 'sent')}><SendIcon class="h-4 w-4" />Sent</Button
					>
					<Button
						variant={current_mailbox == 'drafts' ? 'outline' : 'ghost'}
						size="sm"
						class="justify-start gap-2"
						href="/mail/drafts"
						on:click={() => (current_mailbox = 'drafts')}><FileIcon class="h-4 w-4" />Drafts</Button
					>
					<Button
						variant={current_mailbox == 'trash' ? 'outline' : 'ghost'}
						size="sm"
						class="justify-start gap-2"
						href="/mail/trash"
						on:click={() => (current_mailbox = 'trash')}><Trash2Icon class="h-4 w-4" />Trash</Button
					>
				</div>
			</div>
		{/if}
		<div class="w-full">
			<slot />
		</div>
	</div>
</div>