<script lang="ts">
	import { env } from '$env/dynamic/public';
	import Hamburger from '$lib/components/icons/Hamburger.svelte';
	import { onMount } from 'svelte';
	import type { User } from '$lib/types/index';

	export let isLoggedIn: boolean;
	export let user: User;

	const apiUrl = env.PUBLIC_API_URL;
	let isMobile: boolean;
	let mobileMenuOpen = false;
	let userMenuOpen = false;

	onMount(() => {
		const handleResize = () => {
			isMobile = window.innerWidth > 768;
		};

		window.addEventListener('resize', handleResize);
		return () => window.removeEventListener('resize', handleResize);
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="/styles/navbar.css" />
</svelte:head>

{#if isMobile}
	<nav class="navbar">
		<img class="logo" src="/assets/logo.png" alt="Meme Libre logo" loading="lazy" />
		<ul class="navbar-items">
			<li><a href="/" data-sveltekit-preload-data="hover">Hogar</a></li>
			<li><a href="/upload" data-sveltekit-preload-data="hover">Subir</a></li>
		</ul>
		<ul class="navbar-items user-actions-container">
			{#if isLoggedIn}
				<details class="details">
					<summary>{user.username}</summary>
					<ul class="user-actions">
						<li><a class="user-actions-option" href="/user/put">Cuenta</a></li>
						<li><a class="user-actions-option" href={`${apiUrl}/auth/logout`}>Logout</a></li>
					</ul>
				</details>
			{:else}
				<li><a href={`${apiUrl}/auth/login`}>Login</a></li>
			{/if}
		</ul>
	</nav>
{:else}
	<nav class="navbar-mobile">
		<img class="logo" src="/assets/logo.png" alt="Meme Libre logo" loading="lazy" />
		<details>
			<summary class="summary-mobile"><Hamburger /></summary>
			<ul class="navbar-items-mobile">
				<li><a href="/" data-sveltekit-preload-data="hover">Hogar</a></li>
				<li><a href="/upload" data-sveltekit-preload-data="hover">Subir</a></li>
				{#if isLoggedIn}
					<details class="details-user-mobile">
						<summary>{user.username}</summary>
						<ul class="user-actions-mobile">
							<li><a class="user-actions-option" href="/user/put">Cuenta</a></li>
							<li><a class="user-actions-option" href={`${apiUrl}/auth/logout`}>Logout</a></li>
						</ul>
					</details>
				{:else}
					<li><a href={`${apiUrl}/auth/login`}>Login</a></li>
				{/if}
			</ul>
		</details>
	</nav>
{/if}
