<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import {
		AlignJustifyIcon,
		SearchIcon,
		InboxIcon,
		Trash2Icon,
		SendIcon,
		FileIcon,
		MailIcon,
		CalendarIcon,

		ArrowLeftFromLineIcon,

		ArrowRightFromLineIcon


	} from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';

	var current_mailbox = "inbox";
	var collapsed = true;
</script>

<div class="min-h-screen w-full flex flex-col">
	<div class="border-b px-2 pb-1 pt-2 select-none">
		<div class="flex justify-between">
			<Select.Root selected={{value: "all", label: "All"}}>
				<Select.Trigger class="max-w-44" value="all">
					<Select.Value placeholder="Mail-Account" />
				</Select.Trigger>
				<Select.Content>
					<Select.Item value="all">All</Select.Item>
				</Select.Content>
				<Select.Input value="all" />
			</Select.Root>
			<form class="flex w-full max-w-md items-center space-x-2">
				<Input />
				<Button variant="ghost" size="icon"><SearchIcon class="h-4 w-4" /></Button>
			</form>
			<div>
				<Button variant="outline" size="sm">Settings</Button>
				<Button variant="outline" size="sm">Accounts</Button>
			</div>
		</div>
	</div>
	<div class="flex-grow flex overflow-hidden">
		{#if collapsed}
			<div class="border-r p-2 select-none">
				<Button variant="ghost" size="icon" class="mb-2" on:click={() => collapsed = false}><ArrowRightFromLineIcon class="h-5 w-5" /></Button>
				<div class="grid gap-1 pt-4 border-t">
					<Button variant={current_mailbox == "inbox" ? "outline" : "ghost"} size="icon" href="/mail/inbox" on:click={() => current_mailbox = "inbox"}
						><InboxIcon class="h-4 w-4" /></Button
					>
					<Button variant={current_mailbox == "sent" ? "outline" : "ghost"} size="icon" href="/mail/sent" on:click={() => current_mailbox = "sent"}
						><SendIcon class="h-4 w-4" /></Button
					>
					<Button variant={current_mailbox == "drafts" ? "outline" : "ghost"} size="icon" href="/mail/drafts" on:click={() => current_mailbox = "drafts"}
						><FileIcon class="h-4 w-4" /></Button
					>
					<Button variant={current_mailbox == "trash" ? "outline" : "ghost"} size="icon" href="/mail/trash" on:click={() => current_mailbox = "trash"}
						><Trash2Icon class="h-4 w-4" /></Button
					>
				</div>
			</div>
		{:else}
			<div class="w-full max-w-48 border-r p-4 lg:max-w-sm select-none">
				<div class="pb-4 flex justify-between items-center">
					<h2 class="text-xl font-semibold">Mail</h2>
					<Button variant="ghost" size="icon" on:click={() => collapsed = true}><ArrowLeftFromLineIcon class="h-5 w-5" /></Button>
				</div>
				<div class="grid gap-1">
					<Button variant={current_mailbox == "inbox" ? "outline" : "ghost"} size="sm" class="justify-start gap-2" href="/mail/inbox" on:click={() => current_mailbox = "inbox"}
						><InboxIcon class="h-4 w-4" />Inbox</Button
					>
					<Button variant={current_mailbox == "sent" ? "outline" : "ghost"} size="sm" class="justify-start gap-2" href="/mail/sent" on:click={() => current_mailbox = "sent"}
						><SendIcon class="h-4 w-4" />Sent</Button
					>
					<Button variant={current_mailbox == "drafts" ? "outline" : "ghost"} size="sm" class="justify-start gap-2" href="/mail/drafts" on:click={() => current_mailbox = "drafts"}
						><FileIcon class="h-4 w-4" />Drafts</Button
					>
					<Button variant={current_mailbox == "trash" ? "outline" : "ghost"} size="sm" class="justify-start gap-2" href="/mail/trash" on:click={() => current_mailbox = "trash"}
						><Trash2Icon class="h-4 w-4" />Trash</Button
					>
				</div>
			</div>
		{/if}
		<div class="w-full">
			<slot />
		</div>
	</div>
</div>
