<script lang="ts">
	import { env } from '$env/dynamic/public';
	import Meme from '$lib/components/Meme.svelte';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	const apiUrl = env.PUBLIC_API_URL;
	let isLoading: boolean = false;
	let memes = [...data.memes];
	let observer: IntersectionObserver;
	let sentinel: HTMLDivElement;

	const loadMoreMemes = async (offset: number): Promise<void> => {
		if (isLoading) return;
		isLoading = true;

		try {
			const response = await fetch(`${apiUrl}/meme/get?offset=${offset}`);

			const newMemes = await response.json();

			if (newMemes.length === 0) {
				observer.disconnect();
				window.showNotification('info', 'No hay más memes');
				return;
			}

			memes = [...memes, ...newMemes];

			window.showNotification();
		} catch {
			window.showNotification('error', 'Qué pasó ahora, la puta madre');
		} finally {
			isLoading = false;
		}
	};

	onMount(() => {
		observer = new IntersectionObserver(
			(entries: IntersectionObserverEntry[]) => {
				if (!entries[0].isIntersecting || isLoading || memes.length === 0) return;

				const lastMeme = memes[memes.length - 1];
				if (lastMeme?.id) {
					loadMoreMemes(Number(lastMeme.id));
				}
			},
			{
				root: null,
				rootMargin: '10px',
				threshold: 0
			}
		);

		observer.observe(sentinel);

		return () => {
			if (observer) {
				observer.disconnect();
			}
		};
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="/styles/multiple_meme.css" />
</svelte:head>

{#each memes as meme (meme.id)}
	<Meme {meme} />
{/each}

<div bind:this={sentinel} id="sentinel" data-api-url={data.apiUrl} style="height: 1px;"></div>

{#if isLoading}
	<div class="loading-indicator">Cargando más memes...</div>
{/if}
