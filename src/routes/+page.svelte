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

		MoonIcon,

		SunIcon


	} from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import EmailPreview from '@/custom/components/EmailPreview.svelte';
	import * as Select from '$lib/components/ui/select';
	import CustomLayout from '../lib/custom/layouts/_customLayout.svelte';
	import { logs, theme } from '@/stores/settings';

	function onColorSwitchBtnClicked() {
		if ($theme === 'light') {
			theme.set('dark');
			document.body.classList.add('dark');
		} else {
			theme.set('light');
			document.body.classList.remove('dark');
		}
	}
</script>

<CustomLayout>
	<!-- darkmode toggle -->
	<div class="p-2">
		<Button size="icon" variant="default" class="" on:click={onColorSwitchBtnClicked}>{#if $theme == 'light'}<MoonIcon />{:else}<SunIcon />{/if}</Button>
	</div>
	{#if $logs}
		<div class="p-2">
			{#each $logs as log}
				<div class="p-2 bg-gray-200 dark:bg-gray-800 rounded-lg">
					{log.type}: {log.message}
				</div>
			{/each}
		</div>
	{/if}
</CustomLayout>
