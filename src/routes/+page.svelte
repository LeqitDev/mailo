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
		CalendarIcon
	} from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import EmailPreview from '$lib/components/EmailPreview.svelte';
	import * as Select from '$lib/components/ui/select';
</script>

<div class="flex h-full w-full flex-col">
	<div class="flex w-full justify-between border-b px-2 pb-1 pt-2 select-none">
		<Select.Root>
			<Select.Trigger class="max-w-56">
				<Select.Value placeholder="Mail-Account" />
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="all">All</Select.Item>
			</Select.Content>
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

	<div class="grow-1 flex h-full w-full">
		<div class="h-full w-full max-w-48 border-r p-4 lg:max-w-sm select-none">
			<h2 class="pb-4 text-xl font-semibold">Mail</h2>
			<div class="grid gap-1">
				<Button variant="ghost" size="sm" class="justify-start gap-2"
					><InboxIcon class="h-4 w-4" />Inbox</Button
				>
				<Button variant="ghost" size="sm" class="justify-start gap-2"
					><SendIcon class="h-4 w-4" />Sent</Button
				>
				<Button variant="ghost" size="sm" class="justify-start gap-2"
					><FileIcon class="h-4 w-4" />Drafts</Button
				>
				<Button variant="ghost" size="sm" class="justify-start gap-2"
					><Trash2Icon class="h-4 w-4" />Trash</Button
				>
			</div>
		</div>
		<div class="h-full w-full overflow-y-auto">
		</div>
	</div>
</div>
