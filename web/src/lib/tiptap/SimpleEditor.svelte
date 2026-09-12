<script lang="ts">
	import { onDestroy } from 'svelte';
	import { Editor, Extension } from '@tiptap/core';
	import { StarterKit } from '@tiptap/starter-kit';
	import BubbleMenu from '@tiptap/extension-bubble-menu';
	import * as Y from 'yjs';
	import Collaboration from '@tiptap/extension-collaboration';
	import CollaborationCaret from '@tiptap/extension-collaboration-caret';
	import type { WebsocketProvider } from 'y-websocket';

	let bubbleMenu = $state() as HTMLElement;
	let element = $state() as Element;
	let initialized = false;
	let { doc, provider }: { doc?: Y.Doc; provider?: WebsocketProvider } = $props();
	let extensions: Extension<any, any>[] = [
		StarterKit.configure({ undoRedo: false }),
		BubbleMenu.configure({ element: bubbleMenu })
	];
	let editorState = $state.raw({ editor: null as null | Editor });

	$effect(() => {
		if (doc == undefined || initialized) {
			return;
		}
		editorState = {
			editor: new Editor({
				element: element,
				extensions: [
					StarterKit.configure({ undoRedo: false }),
					BubbleMenu.configure({ element: bubbleMenu }),
					Collaboration.configure({ document: doc }),
					CollaborationCaret.configure({
						provider,
						user: {
							name: 'test-name',
							color: '#8800ff'
						}
					})
				],
				content: '',
				editorProps: {
					attributes: {
						autocomplete: 'off',
						autocorrect: 'off',
						autocapitalize: 'off',
						'aria-label': 'Main content area, start typing to enter text.',
						class: 'simple-editor'
					}
				},
				onTransaction: ({ editor }) => {
					console.log('onTransaction');
					// Update the state signal to force a re-render
					editorState = { editor };
				}
			})
		};
		initialized = true;
	});

	onDestroy(() => {
		editorState.editor?.destroy();
	});
</script>

<div style="position: relative" class="simple-editor-wrapper">
	<div class="fixed-menu">
		<button
			onclick={() => editorState.editor?.chain().focus().toggleHeading({ level: 1 }).run()}
			class:active={editorState.editor?.isActive('heading', { level: 1 })}
		>
			H1
		</button>
		<button
			onclick={() => editorState.editor?.chain().focus().toggleHeading({ level: 2 }).run()}
			class:active={editorState.editor?.isActive('heading', { level: 2 })}
		>
			H2
		</button>
		<button
			onclick={() => editorState.editor?.chain().focus().setParagraph().run()}
			class:active={editorState.editor?.isActive('paragraph')}
		>
			P
		</button>
	</div>

	<div bind:this={element} class="simple-editor-content"></div>
</div>

<style lang="scss">
	@use 'src/lib/styles/tip-tap/paragraph-node.scss' as *;
	@import url('https://fonts.googleapis.com/css2?family=DM+Sans:ital,opsz,wght@0,9..40,100..1000;1,9..40,100..1000&family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap');

	body {
		--tt-toolbar-height: 44px;
		--tt-theme-text: var(--tt-gray-light-900);

		.dark & {
			--tt-theme-text: var(--tt-gray-dark-900);
		}
	}

	body {
		font-family: 'Inter', sans-serif;
		color: var(--tt-theme-text);
		font-optical-sizing: auto;
		font-weight: 400;
		font-style: normal;
		padding: 0;
		overscroll-behavior-y: none;
	}

	html,
	body {
		overscroll-behavior-x: none;
	}

	html,
	body,
	#root,
	#app {
		height: 100%;
		background-color: var(--tt-bg-color);
	}

	::-webkit-scrollbar {
		width: 0.25rem;
	}

	* {
		scrollbar-width: thin;
		scrollbar-color: var(--tt-scrollbar-color) transparent;
	}

	::-webkit-scrollbar-thumb {
		background-color: var(--tt-scrollbar-color);
		border-radius: 9999px;
	}

	::-webkit-scrollbar-track {
		background: transparent;
	}

	.tiptap.ProseMirror {
		font-family: 'DM Sans', sans-serif;
	}

	.simple-editor-wrapper {
		width: 100vw;
		height: 100vh;
		overflow: auto;
	}

	.simple-editor-content {
		max-width: 648px;
		width: 100%;
		margin: 0 auto;
		height: 100%;
		display: flex;
		flex-direction: column;
		flex: 1;
	}

	:global(.simple-editor-content .tiptap.ProseMirror.simple-editor) {
		flex: 1;
		height: 100%;
		padding: 3rem 3rem 30vh;
	}

	@media screen and (max-width: 480px) {
		:global(.simple-editor-content .tiptap.ProseMirror.simple-editor) {
			padding: 1rem 1.5rem 30vh;
		}
	}

	/* Give a remote user a caret */
	:global(.tiptap .collaboration-carets__caret) {
		border-left: 1px solid #0d0d0d;
		border-right: 1px solid #0d0d0d;
		margin-left: -1px;
		margin-right: -1px;
		pointer-events: none;
		position: relative;
		word-break: normal;
	}

	/* Render the username above the caret */
	:global(.tiptap .collaboration-carets__label) {
		border-radius: 3px 3px 3px 0;
		color: #0d0d0d;
		font-size: 12px;
		font-style: normal;
		font-weight: 600;
		left: -1px;
		line-height: normal;
		padding: 0.1rem 0.3rem;
		position: absolute;
		top: -1.4em;
		user-select: none;
		white-space: nowrap;
	}
</style>
