<script lang="ts">
    import { onMount } from "svelte";

  
  let parse = (v: string) => ""
  let value = $state("")
  let output = $derived(parse(value))

  onMount(() => {
    parse = window?.parse
  })
</script>

<main>
  <section>
    <textarea bind:value={value} id="input"></textarea>
  </section>
  <section id="output">
    {output}
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
