<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import { SvelteTheme } from 'svelte-themes';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<SvelteTheme defaultTheme="dark">
	{@render children()}
</SvelteTheme>
