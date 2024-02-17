<script lang="ts">
	import { selected_previews } from "@/stores/emails";
	import { redirect } from "@sveltejs/kit";
	import { Button } from "./ui/button";

	export var new_email: boolean = false;
	export var from: string = 'Giesel, Jürgen';
	export var subject: string =
		'(UE) Programmierung (Übung - Tutorium): Selbstlernmodule "Python4Java" und "C- und Shell-Programmierung"';
	export var body: string =
		'Liebe Studierende, in den Semesterferien (vor Besuch der Vorlesungen "Datenstrukturen und Algorithmen" bzw. "Betriebssysteme und Systemsoftware") sollten Sie zwei Selbstlernmodule absolvieren. Diese bauen auf dem Stoff in der "Programmierung" auf und vermitteln Ihnen die Grundlagen in Python bzw. C. Hier sind zwei kurze Texte mit dem jeweiligen Moodle-Link für die beiden Module:';
	export var time: string = '2 min ago';
	export var id: number

	var checked: boolean = false;
	var redirect_btn: HTMLAnchorElement;

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
</script>

<a class="hidden" href="/mail/view/1" bind:this={redirect_btn}>Redirect</a>
<button
	class="flex w-full select-none justify-between gap-1 border-b p-1 text-left text-xs outline-none bg-"
	class:bg-border={checked}
	on:dblclick={() => {
		redirect_btn.click();
	}}
	on:click={onEmailClicked}
>
	{#if new_email}
		<div class="h-full pt-1">
			<div class="block h-3 w-3 max-w-4 rounded-full bg-blue-500" />
		</div>
	{/if}
	<div class="flex-grow">
		<h2 class="font-bold">{from}</h2>
		<h3 class="line-clamp-1 overflow-ellipsis font-semibold">
			{subject}
		</h3>
		<p class="line-clamp-1 overflow-ellipsis text-muted-foreground">
			{body}
		</p>
	</div>
	<div>
		<p class="w-max text-muted-foreground">{time}</p>
	</div>
</button>
