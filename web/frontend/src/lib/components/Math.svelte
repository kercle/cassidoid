<script lang="ts">
	import type { ExpressionForms } from '$lib/cassida/types/ExpressionForms';
	import katex from 'katex';
	export let expr: ExpressionForms;
	console.log(expr);
</script>

<div class="math-container flex w-full">
	{@html katex.renderToString(expr.latex, {
		throwOnError: false,
		displayMode: true
	})}
	<!-- {@html expr.mathml} -->
</div>

<style>
	.math-container {
		font-size: 0.9rem;
		padding: 0;
		width: 100%;
		overflow-x: auto;
	}
</style>
