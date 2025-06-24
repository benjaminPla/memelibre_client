<script lang="ts">
	import Meme from '$lib/components/Meme.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	let memes = [...data.memes];

	function unsaveMeme(id: string) {
		memes = memes.filter((m) => m.id !== id);
	}
</script>

<svelte:head>
	<link rel="stylesheet" href="/styles/save.css" />
</svelte:head>

{#if memes.length === 0}
	<img src="/assets/not_found.webp" alt="No hay plata meme" loading="lazy" />
	<p>No hay memes</p>
{:else}
	<div class="save-container">
		{#each memes as meme (meme.id)}
			<Meme {meme} {unsaveMeme} />
		{/each}
	</div>
{/if}
