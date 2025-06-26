<script lang="ts">
	import MemeWithComments from '$lib/components/MemeWithComments.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<link rel="stylesheet" href="/styles/single_meme.css" />
</svelte:head>

<MemeWithComments meme={data.meme} />
