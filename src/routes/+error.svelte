<script lang="ts">
	import { page } from '$app/state';
</script>

{#if page.status === 404}
	<img src="/assets/not_found.webp" alt="No hay plata meme" loading="lazy" />
{:else}
	<img src="/assets/error.webp" alt="Alberto Fernández meme" loading="lazy" />
{/if}
<h1>{page.status} - {page.error.message || 'Internal server error'}</h1>
