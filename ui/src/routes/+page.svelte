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

  let astTab = $state(false);

  $effect(() => console.log(output));

  onMount(() => {
    // @ts-ignore
    parse = window?.parse;
  });
</script>

<main>
  <section>
    <textarea placeholder="Write some markdown" bind:value id="input">
    </textarea>
  </section>
  <section id="output">
    <button onclick={() => (astTab = !astTab)}>
      {#if astTab}
        View rendered
      {:else}
        View AST
      {/if}
    </button>
    <div>
      {#if astTab}
        <pre>{output.ast}</pre>
      {:else}
        {@html output.html}
      {/if}
    </div>
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

  button {
    outline: none;
    border: none;
    padding: 12px 24px;
    cursor: pointer;
    width: 150px;
    margin-bottom: 16px;
  }
</style>
