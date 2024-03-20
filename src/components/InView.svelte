<script lang="ts">
    // from https://github.com/the-pudding/svelte-templates/blob/master/templates/InView.svelte
	import { onMount } from 'svelte';

	export let root: Element | Document | null | undefined = undefined;
	export let isInViewProp = false;

	let isInView = false;
	let observer;
	let element: Element;

	$: isInView, (isInViewProp = isInView);

	const onChangeVisibility: IntersectionObserverCallback = (e: { isIntersecting: boolean; }[]) => {
		isInView = e[0].isIntersecting;
	};

	onMount(() => {
		let options = {
			root: root
		};

		observer = new IntersectionObserver(onChangeVisibility, options);
		observer.observe(element);
	});
</script>

<div class="c" bind:this={element}>
	<slot {isInView} />
</div>
