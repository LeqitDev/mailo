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
	import EmailPreview from '$lib/components/EmailPreview.svelte';
	import * as Select from '$lib/components/ui/select';
	import CustomLayout from './_customLayout.svelte';
	import { theme } from '@/stores/settings';

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
</CustomLayout>
