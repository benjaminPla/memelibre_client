<script lang="ts">
	import Meme from '$lib/components/Meme.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<Meme meme={data.meme} />
