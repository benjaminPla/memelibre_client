<script lang="ts">
	import { env } from '$env/dynamic/public';
	import Hamburger from '$lib/components/icons/Hamburger.svelte';
	import { onMount } from 'svelte';
	import type { User } from '$lib/types/index';

	export let isLoggedIn: boolean;
	export let user: User;

	const apiUrl = env.PUBLIC_API_URL;
	let isMobile: boolean;
	let detailsHamburger: HTMLDetailsElement;
	let detailsUser: HTMLDetailsElement;

	function closeHamburger() {
		detailsHamburger.open = false;
	}

	onMount(() => {
		const handleResize = () => {
			isMobile = window.innerWidth < 768;
		};

		function handleDetailsToggle(event: MouseEvent) {
			if (detailsHamburger && !detailsHamburger.contains(event.target as Node)) {
				detailsHamburger.open = false;
			}
			if (detailsUser && !isMobile && !detailsUser.contains(event.target as Node)) {
				detailsUser.open = false;
			}
		}

		isMobile = window.innerWidth < 768;

		document.addEventListener('click', handleDetailsToggle);
		window.addEventListener('resize', handleResize);
		return () => window.removeEventListener('resize', handleResize);
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="/styles/navbar.css" />
</svelte:head>

{#if isMobile}
	<nav class="navbar-mobile">
		<img class="logo logo-mobile" src="/assets/logo_2.png" alt="Meme Libre logo" loading="lazy" />
		<details bind:this={detailsHamburger}>
			<summary class="summary-mobile"><Hamburger /></summary>
			<ul class="navbar-items-mobile">
				<li><a href="/" data-sveltekit-preload-data="hover" on:click={closeHamburger}>Hogar</a></li>
				<li>
					<a href="/upload" data-sveltekit-preload-data="hover" on:click={closeHamburger}>Subir</a>
				</li>
				{#if isLoggedIn}
					<li>
						<a href="/save" data-sveltekit-preload-data="hover" on:click={closeHamburger}
							>Guardados</a
						>
					</li>
					<details class="details-user-mobile">
						<summary>{user.username}</summary>
						<ul class="user-actions-mobile">
							<li>
								<a class="user-actions-option" href="/user/put" on:click={closeHamburger}>Cuenta</a>
							</li>
							<li><a class="user-actions-option" href={`${apiUrl}/auth/logout`}>Logout</a></li>
						</ul>
					</details>
				{:else}
					<li><a href={`${apiUrl}/auth/login`}>Login</a></li>
				{/if}
			</ul>
		</details>
	</nav>
{:else}
	<nav class="navbar">
		<div class="navbar-width">
			<img class="logo logo-desktop" src="/assets/logo_2.png" alt="Meme Libre logo" loading="lazy" />
			<ul class="navbar-items">
				<li><a href="/" data-sveltekit-preload-data="hover">Hogar</a></li>
				<li><a href="/upload" data-sveltekit-preload-data="hover">Subir</a></li>
				{#if isLoggedIn}
					<li><a href="/save" data-sveltekit-preload-data="hover">Guardados</a></li>
				{/if}
			</ul>
			<ul class="navbar-items user-actions-container">
				{#if isLoggedIn}
					<details bind:this={detailsUser} class="details">
						<summary>{user.username}</summary>
						<ul class="user-actions">
							<li>
								<a
									class="user-actions-option"
									href="/user/put"
									on:click={() => (detailsUser.open = false)}>Cuenta</a
								>
							</li>
							<li><a class="user-actions-option" href={`${apiUrl}/auth/logout`}>Logout</a></li>
						</ul>
					</details>
				{:else}
					<li><a href={`${apiUrl}/auth/login`}>Login</a></li>
				{/if}
			</ul>
		</div>
	</nav>
{/if}
