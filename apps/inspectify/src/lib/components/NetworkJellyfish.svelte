<script lang="ts">
  import { mirage } from 'ayu';
  import { onMount } from 'svelte';
  import type { Network } from 'vis-network/esnext';

  interface Props {
    dot: string;
  }

  let { dot }: Props = $props();

  let container: HTMLDivElement | undefined = $state();
  let network_jellyfish: Network | undefined = $state();

  let redraw = $derived(async () => {
    let preDot = dot;
    const vis = await import('vis-network/esnext');
    if (preDot != dot) return;
    const data = vis.parseDOTNetwork(dot);

    data.nodes.forEach((node: any, index: number) => {
      const isBlue = index % 2 === 0;

      node.color = {
        background: mirage.ui.fg.hex(),
        border: isBlue ? '#0077ff' : '#faa54b',
      };

      node.size = 34;
      node.borderWidth = 1;

      node.shadow = {
        enabled: true,
        color: isBlue ? '#0077ff' : '#faa54b',
        size: 25,
        x: 0,
        y: 0,
      };
    });

    //mark accepting states
    data.nodes.forEach((node: any) => {
      if (node.accepting) {
        node.shape = 'star';
        node.borderWidth = 2;
        node.color = {
          ...node.color,
          border: '#FF5555', // accepting
        };
      }
    });

    if (network_jellyfish) {
      network_jellyfish.setData(data);
    } else {
      if (!container) return;

      network_jellyfish = new vis.Network(container, data, {
        // interaction: { zoomView: false },
        nodes: {
          color: {
            background: mirage.ui.fg.hex(),
            border: mirage.ui.fg.hex(),
            highlight: mirage.ui.fg.brighten(1).hex(),
            // background: '#666666',
            // border: '#8080a0',
            // highlight: '#80a0ff',
          },
          font: {
            color: 'white',
          },
          borderWidth: 1,
          shape: 'circle',
          size: 30,
        },
        edges: {
          // color: '#D0D0FF',
          color: mirage.syntax.constant.hex(),
          font: {
            color: 'white',
            strokeColor: '#200020',
            face: 'Menlo, Monaco, "Courier New", monospace',
          },
        },
        autoResize: true,
      });
    }
  });

  onMount(() => {
    if (!container) return;
    const observer = new ResizeObserver(() => {
      requestAnimationFrame(() => {
        if (network_jellyfish) {
          network_jellyfish.fit({ animation: false, maxZoomLevel: 20 });
          network_jellyfish.redraw();
        }
      });
    });
    observer.observe(container);
    return () => observer?.disconnect();
  });

  onMount(() => {
    redraw();
  });

  $effect(() => {
    dot && network_jellyfish && redraw();
  });
</script>

<div class="relative h-full w-full">
  <div class="absolute inset-0" bind:this={container}></div>
</div>
