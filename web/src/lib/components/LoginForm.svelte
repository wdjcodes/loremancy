<script lang="ts">

	import { app } from '$lib/app.svelte';

    let username = $state();
    let password = $state();

    async function loginSubmit(event: SubmitEvent) {
        event.preventDefault();
		if (document.activeElement instanceof HTMLElement) {
			document.activeElement.blur();
		}

        try {
			const response = await fetch('/api/login', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({username, password})
			});

			if (response.ok) {
                app.user = await response.json()
			}
		} catch (error) {
			console.log(error)
		}
    }

</script>

<form class="card bg-surface-100-900 p-4 w-full max-w-md mx-auto space-y-4 mt-6" onsubmit={loginSubmit}>
	<header>
		<h3 class="h3">Login</h3>
	</header>
	<fieldset class="fieldset space-y-2">
		<label class="label">
			<span class="label-text">username</span>
			<input class="input" type="username" bind:value={username}/>
		</label>
        <label class="label">
			<span class="label-text">password</span>
			<input class="input" type="password" bind:value={password}/>
		</label>
	</fieldset>
	<footer class="flex justify-end">
		<button type="submit" class="btn preset-filled">login</button>
	</footer>
</form>