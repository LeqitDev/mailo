<script lang="ts">
	import { selected_previews } from "@/stores/emails";
	import { redirect } from "@sveltejs/kit";
	import { Button } from "../../components/ui/button";
	import { formatTimestamp } from "@/utils";
	import { search_string } from "@/stores/settings";
	import { onMount } from "svelte";
	import * as ContextMenu from "@/components/ui/context-menu";

	export let email: Data.Email;
	export var id: number;
	export let variant: "default" | "compact" = "default";

	var checked: boolean = false;
	var redirect_btn: HTMLAnchorElement;

	const [datePart, timePart] = email.date.split(' ');
    const [year, month, day] = datePart.split('-').map(Number);
    const [hours, minutes, seconds] = timePart.split(':').map(Number);
    const date = new Date(year, month - 1, day, hours, minutes, seconds);

	selected_previews.subscribe((value) => {
		if (value.includes(id)) {
			checked = true;
		} else {
			checked = false;
		}
	});

	function onEmailClicked(e: MouseEvent) {
		if (e.ctrlKey && e.shiftKey) {
			var last_selected = $selected_previews[$selected_previews.length - 1];
			if (last_selected > id) {
				console.log(Array.from({ length: last_selected - id }, (_, i) => id + i))
				$selected_previews = [...$selected_previews, ...Array.from({ length: last_selected - id + 1 }, (_, i) => id + i).reverse()];
			} else {
				console.log(Array.from({ length: id - last_selected }, (_, i) => last_selected + i))
				$selected_previews = [...$selected_previews, ...Array.from({ length: id - last_selected + 1 }, (_, i) => last_selected + i).reverse()];
			}
		} else if (e.ctrlKey) {
			if ($selected_previews.includes(id)) {
				$selected_previews = $selected_previews.filter((value) => value !== id);
			} else {
				$selected_previews = [...$selected_previews, id];
			}
		} else if (e.shiftKey) {
			var last_selected = $selected_previews[$selected_previews.length - 1];
			if (last_selected > id) {
				console.log(Array.from({ length: last_selected - id }, (_, i) => id + i))
				$selected_previews = [...Array.from({ length: last_selected - id + 1 }, (_, i) => id + i).reverse()];
			} else {
				console.log(Array.from({ length: id - last_selected }, (_, i) => last_selected + i))
				$selected_previews = [...Array.from({ length: id - last_selected + 1 }, (_, i) => last_selected + i).reverse()];
			}
		} else {
			$selected_previews = [id];
		}
		console.log($selected_previews);
	}

	let button: HTMLButtonElement;
	let inView: boolean = false;

	onMount(() => {
		if (button) {
			inView = isInView(button);
		}
	});

	function isInView(el: HTMLElement) {
		const rect = el.getBoundingClientRect();
		return (
			rect.top >= 0 &&
			rect.left >= 0 &&
			rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
			rect.right <= (window.innerWidth || document.documentElement.clientWidth)
		);
	}
</script>
{#if email}
	<ContextMenu.Root>
		<ContextMenu.Trigger>
			<button
		class="flex w-full select-none justify-between gap-1 border-b p-1 text-left text-xs outline-none"
		class:bg-border={checked}
		class:hover:bg-muted={!checked}
		on:dblclick={() => {
			redirect_btn.click();
		}}
		on:click={onEmailClicked}
		bind:this={button}
	>
		{#if !email.flags.seen}
			<div class="h-full pt-1">
				<div class="block h-2 w-2 max-w-4 rounded-full bg-blue-500" />
			</div>
		{/if}
		<div class="flex-grow">
			<h2 class="font-bold">{email.sender}</h2>
			<h3 class="line-clamp-1 overflow-ellipsis font-semibold">
				{email.subject}
			</h3>
			<p class="overflow-ellipsis text-muted-foreground" class:line-clamp-1={variant === "compact"} class:line-clamp-2={variant === "default"}>
				{email.body.replaceAll(/\r?\n|\r|\n\r|<br>|<br\>/g, " ").replace(/<\/?[^>]+(>|$)/g, "").replace(/&[^;]+;/g, " ")}
			</p>
		</div>
		<div>
			<p class="w-max text-muted-foreground">{formatTimestamp(date)}</p>
		</div>
	</button>
		</ContextMenu.Trigger>
		<ContextMenu.Content>
			<ContextMenu.Item>
				Mark as read
			</ContextMenu.Item>
		</ContextMenu.Content>
	</ContextMenu.Root>
	<a class="hidden" href="/mail/view/{email.id}" bind:this={redirect_btn}>Redirect</a>
{/if}
