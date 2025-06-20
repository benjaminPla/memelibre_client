<svelte:head>
	<link rel="stylesheet" href="/styles/navbar.css" />
</svelte:head>

<nav class="navbar">
	<img class="logo" src="/assets/logo.png" alt="Meme Libre logo" loading="lazy" />
	<ul class="navbar-items">
		<li><a href="/" data-sveltekit-preload-data="hover">Hogar</a></li>
		<li><a href="/upload" data-sveltekit-preload-data="hover">Subir</a></li>
	</ul>
</nav>
