<script lang="ts">
	import Collaboration from '@tiptap/extension-collaboration';
	import StarterKit from '@tiptap/starter-kit';
	import { onMount } from 'svelte';
	import { createEditor, EditorContent, type Editor } from 'svelte-tiptap';
	import type { Readable } from 'svelte/store';
	import { WebsocketProvider } from 'y-websocket';
	import * as Y from 'yjs';

    let editor = $state() as Readable<Editor>;
	let ydoc: Y.Doc;
    let provider: WebsocketProvider;
    

	onMount(() => {
        ydoc = new Y.Doc();

        provider = new WebsocketProvider('ws://localhost:3000/ws/note/893eaee4-1e83-4bca-8b3e-149082884a0c', 'room-893eaee4-1e83-4bca-8b3e-149082884a0c', ydoc);

		editor = createEditor({
			extensions: [StarterKit.configure({undoRedo: false}), Collaboration.configure({document: ydoc})],
			content: `Hello world!`
		});

		return () => {
			provider.disconnect();
		};
	});
</script>

<EditorContent editor = {$editor}/>
