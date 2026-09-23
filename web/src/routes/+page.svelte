<script lang="ts">
	import { app } from '$lib/app.svelte';
	import CampaignsTable from '$lib/components/CampaignsTable.svelte';
	import LoginForm from '$lib/components/LoginForm.svelte';
</script>

{#if app.user == undefined}
	<LoginForm />
{:else}
	<div class="card bg-surface-100-900 p-4 w-full max-w-lg mx-auto space-y-4 mt-6">
		<CampaignsTable />
	</div>
{/if}
