<script lang="ts">
  import ky from "ky";
  import { onMount } from "svelte";

  export let merge: unknown = "";
  export const url: string = "localhost:8080";

  async function request(): Promise<void> {
    await ky.get(url)
      .json()
      .then((data: unknown) => { 
        merge = data;
      })
      .catch((error: Error) => {
        console.error("Error connecting to localhost:8080: \n" + error.message);
      });
  }

  onMount(request);
</script>
