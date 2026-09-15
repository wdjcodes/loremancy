<script lang="ts">

	let errors = $state({
		user: false,
		email: false,
		password: false,
		verify_password: false,
	})
	let username = $state();
	let email = $state();
	let password = $state();
	let verify_password = $state();

    async function loginSubmit(event: SubmitEvent) {
        event.preventDefault();

		let err = false;

		if (!username) {
			errors.user = true;
			err = true
		}

		if (!email) {
			errors.email = true;
			err = true
		}

		if (!password || !verify_password || password !== verify_password) {
			errors.password = true;
			errors.verify_password = true;
			err = true
		} 

		if (err) {
			return
		}


        try {
			const response = await fetch('/api/register', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({username, email, password})
			});

			if (response.ok) {
				console.log(response.body)
			}
		} catch (error) {
			console.log(error)
		}
    }

</script>

<form class="card bg-surface-100-900 p-4 w-full max-w-md mx-auto space-y-4 mt-6" onsubmit={loginSubmit}>
	<header>
		<h3 class="h3">Signup</h3>
	</header>
	<fieldset class="fieldset space-y-2">
		<label class="label">
			<span class="label-text">username</span>
			<input class="input" type="username" bind:value={username}/>
		</label>
        <label class="label">
			<span class="label-text">email</span>
			<input class="input" type="email" bind:value={email}/>
		</label>
        <label class="label">
			<span class="label-text" class:input-error={errors.password}>password</span>
			<input class="input" type="password" bind:value={password} />
		</label>
        <label class="label">
			<span class="label-text">verify password</span>
			<input class="input" class:input-error={errors.verify_password} type="password" bind:value={verify_password}/>
		</label>
	</fieldset>
	<footer class="flex justify-end">
		<button type="submit" class="btn preset-filled">login</button>
	</footer>
</form>