<script lang="ts">
  import Env from '$lib/components/Env.svelte';
  import StandardInput from '$lib/components/StandardInput.svelte';
  import { Io } from '$lib/io.svelte';

  const io = new Io('HelloWorld', { source: '' });
</script>

<Env {io}>
  {#snippet inputView()}
    <StandardInput analysis="HelloWorld" code="source" {io} />
  {/snippet}
  {#snippet outputView({ output, referenceOutput })}
    <p class="glow-text">
      {#each output.transformed.split(' ') as word, i}
        <span class={i % 2 === 1 ? 'highlight' : ''}>{word}</span>{' '}
      {/each}
    </p>
  {/snippet}
</Env>

<style>
  p.glow-text {
    font-family: 'Fira Code', monospace;
    font-size: 1.5rem;
    color: #fff;
    text-align: center;
  }

  /* Neon glow for every other word */
  .highlight {
    text-transform: uppercase;
    color: #faa54b; /* cyan glow */
    font-weight: bold;
    text-shadow: 0 0 5px #faa54b;
  }
</style>
