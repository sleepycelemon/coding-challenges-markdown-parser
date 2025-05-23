<script lang="ts">
  type ParserResult = {
    html: String;
    ast: String;
  };

  import { onMount } from "svelte";

  let parse = (v: string): ParserResult => ({
    ast: "",
    html: "",
  });

  let value = $state("");
  let output: ParserResult = $derived(parse(value));

  let astTab = $state(true)

  $effect(() => console.log(output));

  onMount(() => {
    // @ts-ignore
    parse = window?.parse;
  });
</script>

<main>
  <section>
    <textarea bind:value id="input"></textarea>
  </section>
  <section id="output">
    <button onclick={() => astTab = !astTab}>Toggle Tab</button>
    <br />
    {#if astTab}
      <pre>{output.ast}</pre>
    {:else}
      {@html output.html}
    {/if}
  </section>
</main>

<style>
  :global(html, body) {
    padding: 0;
    margin: 0;
  }

  main {
    width: 100vw;
    height: 100vh;
    display: flex;
    font-size: 20px;
    background-color: gainsboro;
    gap: 4px;
  }

  section {
    flex: 1;
    box-sizing: border-box;
    background-color: white;
    padding: 16px;
  }

  section > textarea {
    height: 100%;
    width: 100%;
    border: none;
    outline: none;
    font-size: inherit;
    box-sizing: border-box;
    resize: none;
  }
</style>
