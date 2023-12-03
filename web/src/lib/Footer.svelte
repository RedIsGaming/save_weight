<script lang="ts">
  import { Icon, AtSymbol } from "svelte-hero-icons";

  export function current_year(): number {
    return new Date().getFullYear();
  }
</script>

<footer class="w-full h-10 bg-transparant bottom-0 left-0 fixed border-t-2 dark:bg-transparant">
  <section class="h-10 flex justify-center items-center text-gray-950 dark:text-white">
    <Icon src="{AtSymbol}" size="20" solid />
    <p>{current_year()} No Rights Reserved!</p>
  </section>
</footer>
