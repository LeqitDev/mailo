<script lang="ts">
    import * as RadioGroup from '@/components/ui/radio-group';
    import { placeholderEmail, settings } from '@/store';
    import { buttonVariants } from '@/components/ui/button';
    import { Label } from '@/components/ui/label';
    import { CalendarIcon, HomeIcon, MailIcon, SettingsIcon } from 'lucide-svelte';
    import { cn } from '@/utils';
	import EmailPreview from './EmailPreview.svelte';

    export let size: "md" | "lg" = "md";
	export let title = "Change your theme";

    let lg_size_classes = "h-[7.5rem] w-screen max-w-lg";
	let md_size_classes = "h-20 w-80";
    let sizes = {
        md: md_size_classes,
        lg: lg_size_classes
    };
	let placeholder = "Anonymous User <anon.ymous@private.com>";

	function onThemeSelected(new_theme: string) {
		if (new_theme !== 'light' && new_theme !== 'dark') return;
		settings.update((settings) => {
			settings.theme = new_theme;
			return settings;
		});
	}
</script>

<div>
	<p class="mb-2 text-xl font-semibold">{title}</p>
	<RadioGroup.Root value={$settings.theme} class={`${size == "lg" ? "flex" : ""} gap-4`} onValueChange={onThemeSelected}>
		<div class="flex space-x-2">
			<RadioGroup.Item value="light" id="light" />
			<Label for="light" class="cursor-pointer group"
				><span class="font-bold group-hover:underline">Light</span>
				<div
					class={`mt-2 overflow-clip ${sizes[size]} rounded-sm border bg-background dark:border-muted-foreground dark:bg-foreground`}
				>
					<div class="flex h-full gap-2 w-screen">
						<div class="grid h-full gap-1 border-r p-2 dark:border-muted-foreground">
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
							<div
								class={cn(
									buttonVariants({
										variant: 'ghost',
										size: 'icon',
										className: 'hover:bg-[hsl(0_0%_90.1%)] dark:text-background'
									})
								)}
							>
								<CalendarIcon />
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
								<SettingsIcon />
							</div>
						</div>
						<div class="p-2 w-screen">
							<h1
								class="text-hidden mb-4 w-full overflow-hidden whitespace-nowrap text-2xl font-semibold dark:text-background"
							>
								Welcome to your Dashboard.
							</h1>
							<div class="pt-1">
								<p class="text-lg mb-2 dark:text-background">New & recent emails</p>
							</div>
							<div class="flex hover:bg-border hover:dark:bg-[hsl(0_0%_89.8%)] pb-5 p-1 pr-20 w-full">
								<span class="bg-blue-500 rounded-full h-2 w-2 mt-1 mr-2"></span>
								<p class="text-xs font-bold grow dark:text-background">{placeholder}</p>
							</div>
						</div>
					</div>
				</div>
			</Label>
		</div>
		<div class="flex space-x-2">
			<RadioGroup.Item value="dark" id="dark" />
			<Label for="dark" class="cursor-pointer group"
				><span class="font-bold group-hover:underline">Dark</span>
				<div
					class={`mt-2 ${sizes[size]} overflow-clip rounded-sm border bg-foreground dark:bg-background`}
				>
					<div class="flex h-full gap-2">
						<div class="grid h-full gap-2 border-r border-muted-foreground p-2 dark:border-border">
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
								<CalendarIcon />
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
								<SettingsIcon />
							</div>
						</div>
						<div class="p-2 w-screen">
							<h1
								class="text-hidden mb-4 w-full overflow-hidden whitespace-nowrap text-2xl font-semibold text-background dark:text-foreground"
							>
								Welcome to your Dashboard.
							</h1>
							<div class="pt-1">
								<p class="text-lg mb-2 text-background dark:text-foreground">New & recent emails</p>
							</div>
							<div class="flex hover:dark:bg-border hover:bg-[hsl(0_0%_14.9%)] pb-5 p-1 pr-20 w-full">
								<span class="bg-blue-500 rounded-full h-2 w-2 mt-1 mr-2"></span>
								<p class="text-xs font-bold grow text-background dark:text-foreground">{placeholder}</p>
							</div>
						</div>
					</div>
				</div>
			</Label>
		</div>
	</RadioGroup.Root>
</div>
