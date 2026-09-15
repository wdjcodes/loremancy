<script lang="ts">
	import LoginForm from "$lib/LoginForm.svelte";


    let user_info = $state();

</script>

{#if user_info == undefined }
<LoginForm bind:user_info={user_info}/>
{:else}
{JSON.stringify(user_info)}
{/if}