<script lang="ts">
	import type { ExpressionForms } from '$lib/cassida/types/ExpressionForms';
	import katex from 'katex';
	export let expr: ExpressionForms;
	console.log(expr);
</script>

<div class="math-container flex w-full" style="font-size: 0.9rem;">
	{@html katex.renderToString(expr.latex, {
		throwOnError: false,
		displayMode: true
	})}
	<!-- {@html expr.mathml} -->
</div>

<style>
	.math-container {
		padding: 1em 0;
		width: 100%;
		overflow-x: auto;
	}
</style>
