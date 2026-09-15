<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { SvelteTheme } from 'svelte-themes';
	import { AppBar } from '@skeletonlabs/skeleton-svelte';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<AppBar>
	<AppBar.Toolbar >
		<!-- <AppBar.Lead /> -->
		<AppBar.Headline>
			<p>loremancy</p>
		</AppBar.Headline>
		<!-- <AppBar.Trail /> -->
	</AppBar.Toolbar>
</AppBar>
 {@render children()}
