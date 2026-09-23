<script lang="ts">
	import { goto } from '$app/navigation';
	import type { Campaign } from '$lib/schema';
	import { onMount } from 'svelte';

	let campaigns: Campaign[] = $state([]);
	let creating = $state(false);
	let new_campaign = $state('');

	onMount(() => {
		const fetchData = async () => {
			let response = await fetch('/api/campaigns', {
				method: 'GET',
				headers: { 'Content-Type': 'application/json' }
			});

			if (response.ok) {
				campaigns = await response.json();
				console.log('Campaigns: ' + campaigns);
			}
		};
		fetchData();
	});

	async function createCampaign(event: SubmitEvent) {
		event.preventDefault();

		try {
			let response = await fetch('/api/campaign', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ name: new_campaign, members: [] })
			});

			if (!response.ok) {
				console.log('Error creating campaign' + (await response.json()));
			}
			response = await fetch('/api/campaigns', {
				method: 'GET',
				headers: { 'Content-Type': 'application/json' }
			});

			if (response.ok) {
				campaigns = await response.json();
				new_campaign = '';
				creatingOff();
			}
		} catch (error) {
			console.log(error);
		}
	}

	function creatingOn() {
		creating = true;
	}

	function creatingOff() {
		creating = false;
	}

	function autofocus(node: HTMLElement) {
		node.focus();
	}
</script>

<div class="table-wrap">
	<table class="table table-zebra caption-bottom">
		<thead>
			<tr>
				<th>
					<div class="flex flex-row justify-between">
						<div>Campaigns</div>
						<button
							type="button"
							class="btn-icon preset-filled"
							title="Create Campaign"
							aria-label="Create Campaign"
							onclick={creatingOn}
						>
							+
						</button>
					</div>
				</th>
			</tr>
		</thead>
		<tbody class="[&>tr]:hover:preset-tonal-brand">
			{#if creating}
				<tr>
					<td>
						<form onsubmit={createCampaign}>
							<input
								use:autofocus
								type="text"
								class="w-full input"
								onblur={creatingOff}
								bind:value={new_campaign}
							/>
						</form>
					</td>
				</tr>
			{/if}
			{#each campaigns as campaign (campaign.id)}
				<tr onclick={() => goto('/campaign/' + campaign.id)}>
					<td>{campaign.name}</td>
				</tr>
			{/each}
			{#if campaigns.length == 0 && !creating}
				<tr onclick={creatingOn}>
					<td>start an adventure</td>
				</tr>
			{/if}
		</tbody>
	</table>
</div>
