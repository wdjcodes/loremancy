<script lang="ts">
	import { onMount } from 'svelte';
	import { WebsocketProvider } from 'y-websocket';
	import * as Y from 'yjs';
	import SimpleEditor from '$lib/components/tiptap/SimpleEditor.svelte';

	let ydoc: Y.Doc | undefined = $state();
	let provider: WebsocketProvider | undefined = $state();

	onMount(() => {
		ydoc = new Y.Doc();

		provider = new WebsocketProvider(
			'ws://localhost:5173/ws/note',
			'893eaee4-1e83-4bca-8b3e-149082884a0c',
			ydoc
		);

		return () => {
			provider?.disconnect();
		};
	});
</script>

<!-- <EditorContent editor={$editor} /> -->
<SimpleEditor doc={ydoc} provider={provider} />