<script lang="ts">
  import Env from '$lib/components/Env.svelte';
  import StandardInput from '$lib/components/StandardInput.svelte';
  import { Io } from '$lib/io.svelte';

  const io = new Io('Automata', { regex: 'skip' });
</script>

<Env {io}>
  {#snippet inputView()}
    <StandardInput analysis="Automata" code="regex"{io} />
  {/snippet}
  {#snippet outputView({ output, referenceOutput })}
  <p>{output.dot}</p>
  {/snippet}
</Env>
