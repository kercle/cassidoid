<script lang="ts">
	import type { ExecutionResult } from '$lib/cassida/types/ExecutionResult';
	import HelpBuiltin from './help/HelpBuiltin.svelte';
	import HelpTableOfContents from './help/HelpTableOfContents.svelte';
	import Math from './Math.svelte';
	import Plot from './Plot.svelte';

	export let index: number = 1;
	export let entry: ExecutionResult | undefined = undefined;

	const getRawInput = (entry: ExecutionResult | undefined) => {
		if (!entry) {
			return null;
		}

		if ('input' in entry) {
			return entry.input;
		} else {
			return null;
		}
	};
</script>

<div class="group w-full">
	<div class="bg-base-200 relative">
		<div class="flex flex-row">
			<div class="bg-base-200 text-info-content flex w-20 items-center justify-center">
				(%i{index})
			</div>
			<div class="bg-base-200 overflow-x-auto pt-3 pb-2 pl-6">
				{getRawInput(entry)}
			</div>
		</div>
	</div>

	{#if entry?.type == 'expression'}
		<div class="flex flex-row">
			<div class="bg-base-200 text-success-content flex w-20 items-center justify-center">
				(%o{index})
			</div>
			<div class="border-base-200 w-full overflow-x-auto border pl-6">
				<Math expr={entry.output.latex} />
			</div>
		</div>
	{:else if entry?.type == 'plot'}
		<div class="flex flex-row">
			<div class="bg-base-200 text-success-content flex w-20 items-center justify-center">
				(%o{index})
			</div>
			<div class="border-base-200 w-full overflow-x-auto border pl-6">
				<Plot data={entry.data} />
			</div>
		</div>
	{:else if entry?.type == 'helpTableOfContents'}
		<HelpTableOfContents builtins={entry.builtins} />
	{:else if entry?.type == 'helpBuiltin'}
		<HelpBuiltin
			title={entry.title}
			patterns={entry.patterns}
			examples={entry.examples}
			related={entry.related}
		/>
	{:else}
		<p>Unknown server message.</p>
	{/if}
</div>
