<script lang="ts">
	import * as RadioGroup from '@/components/ui/radio-group';
	import { placeholderEmail, settings } from '@/store';
	import { buttonVariants } from '@/components/ui/button';
	import { Label } from '@/components/ui/label';
	import { CalendarIcon, HomeIcon, MailIcon, SettingsIcon } from 'lucide-svelte';
	import { cn } from '@/utils';
	import EmailPreview from './EmailPreview.svelte';

	export let size: 'md' | 'lg' = 'md';
	export let title = 'Change your theme';
	export let minimal = false;

	let lg_size_classes = 'h-[7.5rem] w-96 max-w-lg';
	let md_size_classes = 'h-20 w-96';
	let sizes = {
		md: md_size_classes,
		lg: lg_size_classes
	};
	let placeholder = 'Anonymous User <anon.ymous@private.com>';

	function onThemeSelected(new_theme: string) {
		if (new_theme !== 'light' && new_theme !== 'dark') return;
		settings.update((settings) => {
			settings.theme = new_theme;
			return settings;
		});
	}
</script>

<!-- 24 rem -->
<div>
	<p class="mb-2 text-xl font-semibold">{title}</p>
	<RadioGroup.Root
		value={$settings.theme}
		class={`${size == 'lg' ? 'flex' : ''} gap-4`}
		onValueChange={onThemeSelected}
	>
		<div class="flex space-x-2">
			<RadioGroup.Item value="light" id="light" class="sr-only" />
			<Label for="light" class="cursor-pointer">
				<div
					class={`mt-2 overflow-clip ${sizes[size]} flex rounded-sm border bg-background dark:border-muted-foreground dark:bg-foreground dark:outline-none outline outline-2 outline-blue-400`}
				>
					<div
						class="flex h-screen w-6 flex-col items-center gap-1 border-r pt-1 dark:bg-foreground"
					>
						<!-- Sidebar -->
						<div class="h-4 w-4 rounded-sm bg-foreground dark:bg-[hsl(0_0%_3.9%)] p-1">
						</div>
						<div class="h-4 w-4 rounded-sm bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
						<div class="h-4 w-4 rounded-sm bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
                        <div class="h-4 w-4 rounded-sm bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
					</div>
					<div class="w-64 p-2">
                        <!-- Dashboard welcome phrase -->
						<div class="h-2.5 w-28 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
						<div class="mt-2 flex justify-between">
							<!-- New & recent emails -->
							<div class="h-1.5 w-16 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
							<div class="h-1.5 w-16 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
						</div>
						<div class="mt-1">
							<div class="mt-0.5">
								<!-- Email preview -->
								<div class="flex justify-between">
									<div class="flex gap-1">
										<div class="h-1 w-1 rounded-full bg-blue-500"></div>
										<div class="h-1 w-20 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
									</div>
									<div class="h-1 w-8 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
								</div>
								<div class="mt-0.5 pl-2">
									<div class="mb-0.5 h-1 w-16 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
									<div class="h-1 w-54 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
								</div>
							</div>
							<div class="mt-1">
								<!-- Email preview -->
								<div class="flex justify-between">
									<div class="flex gap-1">
										<div class="h-1 w-1 rounded-full bg-blue-500"></div>
										<div class="h-1 w-20 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
									</div>
									<div class="h-1 w-8 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
								</div>
								<div class="mt-0.5 pl-2">
									<div class="mb-0.5 h-1 w-16 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
									<div class="h-1 w-54 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
								</div>
							</div>
							<div class="mt-1">
								<!-- Email preview -->
								<div class="flex justify-between">
									<div class="flex gap-1">
										<div class="h-1 w-1 rounded-full bg-blue-500"></div>
										<div class="h-1 w-20 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
									</div>
									<div class="h-1 w-8 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
								</div>
								<div class="mt-0.5 pl-2">
									<div class="mb-0.5 h-1 w-16 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
									<div class="h-1 w-54 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
								</div>
							</div>
							<div class="mt-1">
								<!-- Email preview -->
								<div class="flex justify-between">
									<div class="flex gap-1">
										<div class="h-1 w-1 rounded-full bg-blue-500"></div>
										<div class="h-1 w-20 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
									</div>
									<div class="h-1 w-8 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
								</div>
								<div class="mt-0.5 pl-2">
									<div class="mb-0.5 h-1 w-16 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
									<div class="h-1 w-54 rounded-full bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
								</div>
							</div>
						</div>
					</div>
                    <!-- Left side accounts & theme -->
                    <div class="w-24">
                        <!-- Accounts -->
                        <div class="flex justify-between items-center mt-6">
                            <div class="h-1.5 w-16 rounded-full bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
                            <div class="h-3 w-3 rounded-sm bg-muted dark:bg-[hsl(0_0%_90.1%)]"></div>
                        </div>
                        <div class="flex justify-between mt-1.5">
                            <div class="flex items-center">
                                <div class="rounded-full bg-blue-300 w-4 h-4"></div>
                                <div class="pl-1">
                                    <div class="rounded-full h-1 w-10 bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
                                    <div class="rounded-full h-1 w-14 bg-muted dark:bg-[hsl(0_0%_90.1%)] mt-1"></div>
                                </div>
                            </div>
                            <div class="rounded-sm bg-muted dark:bg-[hsl(0_0%_90.1%)] h-3 w-3"></div>
                        </div>
                        <div class="flex justify-between mt-1">
                            <div class="flex items-center">
                                <div class="rounded-full bg-blue-300 w-4 h-4"></div>
                                <div class="pl-1">
                                    <div class="rounded-full h-1 w-10 bg-foreground dark:bg-[hsl(0_0%_3.9%)]"></div>
                                    <div class="rounded-full h-1 w-14 bg-muted dark:bg-[hsl(0_0%_90.1%)] mt-1"></div>
                                </div>
                            </div>
                            <div class="rounded-sm bg-muted dark:bg-[hsl(0_0%_90.1%)] h-3 w-3"></div>
                        </div>
                        <!-- Theme -->
                        <div class="rounded-full w-14 h-1.5 bg-foreground dark:bg-[hsl(0_0%_3.9%)] mt-3"></div>
                        <div class="rounded-sm border outline-1 outline outline-blue-300 dark:outline-none h-5 w-24 mt-1 dark:border-muted-foreground"></div>
                    </div>
				</div>
                <p class="w-96 text-center pt-2">Light</p>
			</Label>
		</div>
		<div class="flex space-x-2">
			<RadioGroup.Item value="dark" id="dark" class="sr-only"/>
			<Label for="dark" class="cursor-pointer"
				>
				<div
					class={`mt-2 overflow-clip ${sizes[size]} flex rounded-sm border bg-foreground dark:border-muted-foreground dark:bg-background dark:outline outline-2 outline-blue-400`}
				>
					<div
						class="flex h-screen w-6 flex-col items-center gap-1 border-r pt-1 bg-foreground dark:bg-background"
					>
						<!-- Sidebar -->
						<div class="h-4 w-4 rounded-sm dark:bg-foreground bg-[hsl(0_0%_98%)] p-1">
						</div>
						<div class="h-4 w-4 rounded-sm dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
						<div class="h-4 w-4 rounded-sm dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
                        <div class="h-4 w-4 rounded-sm dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
					</div>
					<div class="w-64 p-2">
                        <!-- Dashboard welcome phrase -->
						<div class="h-2.5 w-28 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
						<div class="mt-2 flex justify-between">
							<!-- New & recent emails -->
							<div class="h-1.5 w-16 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
							<div class="h-1.5 w-16 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
						</div>
						<div class="mt-1">
							<div class="mt-0.5">
								<!-- Email preview -->
								<div class="flex justify-between">
									<div class="flex gap-1">
										<div class="h-1 w-1 rounded-full bg-blue-500"></div>
										<div class="h-1 w-20 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
									</div>
									<div class="h-1 w-8 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
								</div>
								<div class="mt-0.5 pl-2">
									<div class="mb-0.5 h-1 w-16 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
									<div class="h-1 w-54 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
								</div>
							</div>
							<div class="mt-1">
								<!-- Email preview -->
								<div class="flex justify-between">
									<div class="flex gap-1">
										<div class="h-1 w-1 rounded-full bg-blue-500"></div>
										<div class="h-1 w-20 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
									</div>
									<div class="h-1 w-8 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
								</div>
								<div class="mt-0.5 pl-2">
									<div class="mb-0.5 h-1 w-16 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
									<div class="h-1 w-54 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
								</div>
							</div>
							<div class="mt-1">
								<!-- Email preview -->
								<div class="flex justify-between">
									<div class="flex gap-1">
										<div class="h-1 w-1 rounded-full bg-blue-500"></div>
										<div class="h-1 w-20 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
									</div>
									<div class="h-1 w-8 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
								</div>
								<div class="mt-0.5 pl-2">
									<div class="mb-0.5 h-1 w-16 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
									<div class="h-1 w-54 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
								</div>
							</div>
							<div class="mt-1">
								<!-- Email preview -->
								<div class="flex justify-between">
									<div class="flex gap-1">
										<div class="h-1 w-1 rounded-full bg-blue-500"></div>
										<div class="h-1 w-20 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
									</div>
									<div class="h-1 w-8 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
								</div>
								<div class="mt-0.5 pl-2">
									<div class="mb-0.5 h-1 w-16 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
									<div class="h-1 w-54 rounded-full dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
								</div>
							</div>
						</div>
					</div>
                    <!-- Left side accounts & theme -->
                    <div class="w-24">
                        <!-- Accounts -->
                        <div class="flex justify-between items-center mt-6">
                            <div class="h-1.5 w-16 rounded-full dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
                            <div class="h-3 w-3 rounded-sm dark:bg-muted bg-[hsl(0_0%_14.9%)]"></div>
                        </div>
                        <div class="flex justify-between mt-1.5">
                            <div class="flex items-center">
                                <div class="rounded-full bg-blue-300 w-4 h-4"></div>
                                <div class="pl-1">
                                    <div class="rounded-full h-1 w-10 dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
                                    <div class="rounded-full h-1 w-14 dark:bg-muted bg-[hsl(0_0%_14.9%)] mt-1"></div>
                                </div>
                            </div>
                            <div class="rounded-sm dark:bg-muted bg-[hsl(0_0%_14.9%)] h-3 w-3"></div>
                        </div>
                        <div class="flex justify-between mt-1">
                            <div class="flex items-center">
                                <div class="rounded-full bg-blue-300 w-4 h-4"></div>
                                <div class="pl-1">
                                    <div class="rounded-full h-1 w-10 dark:bg-foreground bg-[hsl(0_0%_98%)]"></div>
                                    <div class="rounded-full h-1 w-14 dark:bg-muted bg-[hsl(0_0%_14.9%)] mt-1"></div>
                                </div>
                            </div>
                            <div class="rounded-sm dark:bg-muted bg-[hsl(0_0%_14.9%)] h-3 w-3"></div>
                        </div>

                        <!-- Theme -->
                        <div class="rounded-full w-14 h-1.5 dark:bg-foreground bg-[hsl(0_0%_98%)] mt-3"></div>
                        <div class="rounded-sm border border-[hsl(0_0%_14.9%)] h-6 w-24 mt-1 dark:border-muted bg-background  outline-1 outline outline-blue-300 dark:outline-none"></div>
                    </div>
				</div>
                <p class="w-96 text-center pt-2">Dark</p>
			</Label>
		</div>
	</RadioGroup.Root>
</div>
